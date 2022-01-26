use crate::modifications::{Modification, Reduction};
use crate::reducers::Reduce;
use pnets::standard::Net;
use pnets_tina::ExporterBuilder;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

#[derive(Deserialize, Serialize, Debug)]
struct Member {
    item: String,
    weight: isize,
}

#[derive(Deserialize, Serialize, Debug)]
struct Invariant {
    #[serde(rename = "const")]
    constant: isize,
    eqn: Vec<Member>,
}

/// Remove invariant with struct -R
pub struct InvariantReducer;

impl Reduce<Net> for InvariantReducer {
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        let timeout: u64 = env::var("STRUCT_TIMEOUT")
            .unwrap_or("a".to_string())
            .parse()
            .unwrap_or(u64::MAX);
        let mut child = Command::new("struct")
            .arg("-j3")
            .arg("-P")
            .arg("-4ti2")
            .arg("-R")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let child_stdin = child.stdin.as_mut().unwrap(); // TODO: Error
                                                         // Write net to stdout
                                                         // Create a new net without disconnected places and transitions
        let (new_net, _transition_map, place_map) = net.new_without_disconnected();
        ExporterBuilder::new(child_stdin)
            .with_disconnected_transitions(true)
            .with_all_places(true)
            .build()
            .export(&(&new_net).into())
            .unwrap();
        let mut stdout = child.stdout.take().unwrap();

        let status = child.wait_timeout(Duration::from_millis(timeout));
        match status {
            Ok(None) => {
                return;
            }
            _ => {}
        }
        // Read input
        let mut out_string = String::new();

        stdout.read_to_string(&mut out_string).unwrap();
        match serde_json::from_str::<Vec<Invariant>>(&*out_string) {
            Err(_) => {}
            Ok(equations) => {
                for eq in equations.iter() {
                    // Recherche du seul (si il existe) coefficient à -1
                    if let Some(equals) = eq.eqn.iter().filter(|&m| m.weight == -1).next() {
                        for member in &eq.eqn {
                            if member.weight == -1 {
                                net.delete_place(
                                    place_map[new_net
                                        .get_index_by_name(&member.item)
                                        .unwrap()
                                        .as_place()
                                        .unwrap()],
                                );
                            }
                        }
                        modifications.push(Modification::Reduction(Reduction {
                            equals_to: eq
                                .eqn
                                .iter()
                                .filter_map(|m| {
                                    if m.weight != -1 {
                                        Some((
                                            place_map[new_net
                                                .get_index_by_name(&m.item)
                                                .unwrap()
                                                .as_place()
                                                .unwrap()],
                                            m.weight,
                                        ))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                            constant: eq.constant,
                            deleted_places: vec![(
                                place_map[new_net
                                    .get_index_by_name(&equals.item)
                                    .unwrap()
                                    .as_place()
                                    .unwrap()],
                                1,
                            )],
                        }))
                    }
                }
            }
        }
    }
}
