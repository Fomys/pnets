use crate::modifications::{Modification, Reduction};
use crate::reducers::Reduce;
use pnets::standard::Net;
use pnets_tina::ExporterBuilder;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

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
                                                         // Write network to stdout
                                                         // Create a new network without disconnected places and transitions
        let (new_net, _transition_map, place_map) = net.new_without_disconnected();
        ExporterBuilder::new(child_stdin)
            .with_disconnected_transitions(true)
            .with_all_places(true)
            .build()
            .export(&(&new_net).into())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        // Read input
        let out_string = String::from_utf8(output.stdout).unwrap();
        let equations: Vec<Invariant> = serde_json::from_str(&*out_string).unwrap();
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
