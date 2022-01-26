use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufReader, ErrorKind};
use std::process::exit;

use clap::{App, Arg};
use pnets::NodeId;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Net print")
        .version("1.0")
        .author("Louis C. <louis.chauvet@free.fr>")
        .about("Print a petri net from .net file")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("CONTENT")
                .help("Print net content")
                .short("c")
                .long("content"),
        )
        .get_matches();

    let net = match matches.value_of("INPUT") {
        None | Some("-") => pnets_tina::Parser::new(BufReader::new(stdin())).parse()?,
        Some(file) => pnets_tina::Parser::new(BufReader::new(match File::open(file) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                println!("File {} does not exists.", file);
                exit(1);
            }
            Err(e) => return Err(e.into()),
        }))
        .parse()?,
    };

    println!("Print petri net: {}", net.name);
    println!(
        "There is {} places and {} transitions.",
        net.places.len(),
        net.transitions.len()
    );
    println!(
        "There are {} transitions with priorities.",
        net.transitions
            .iter()
            .filter(|&v| !v.priorities.is_empty())
            .count()
    );

    if matches.is_present("CONTENT") {
        println!(
            "Transitions: {:?}",
            net.transitions
                .iter()
                .map(|tr| net.get_name_by_index(&NodeId::Transition(tr.id())))
        );
        println!(
            "Places: {:?}",
            net.places
                .iter()
                .map(|pl| net.get_name_by_index(&NodeId::Place(pl.id())))
        );
    }
    Ok(())
}
