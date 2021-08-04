use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{stdout, BufReader, Write};
use std::time::SystemTime;

use clap::{App, Arg, ArgMatches};
use log::info;

use pnets::standard::Net;
use pnets::NodeId;
use pnets_shrunk::modifications::Modification;
use pnets_shrunk::reducers::standard::{
    IdentityPlaceReducer, IdentityTransitionReducer, ParallelSmartReducer, PseudoStart, R7Reducer,
    RLReducer, SimpleChainReducer, SimpleLoopAgglomeration, WeightSimplification,
};
use pnets_shrunk::reducers::{Chain5Reducer, ChainReducer, LoopReducer, Reduce, SmartReducer};
use pnets_tina::ExporterBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("PNet reducer")
        .version("1.0")
        .author("Louis C. <louis.chauvet@free.fr>")
        .about("Reduce a petri network")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .short("i")
                .takes_value(true)
                .long("input"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CLEAN")
                .help("Remove all disconnected transitions and places for final output")
                .long("clean"),
        )
        .arg(
            Arg::with_name("EQUATIONS")
                .help("Print equtions with output")
                .long("equations"),
        )
        .arg(
            Arg::with_name("RENUMBER")
                .help("Write renumbered original network")
                .long("renumber"),
        )
        .get_matches();

    info!("Start parsing.");
    let now = SystemTime::now();
    let mut net = Net::from(match matches.value_of("INPUT") {
        Some("-") | None => pnets_tina::Parser::new(BufReader::new(io::stdin())).parse()?,
        Some(f) => pnets_tina::Parser::new(BufReader::new(File::open(f)?)).parse()?,
    });
    info!("Parsing done: {:?}", now.elapsed()?);
    info!("Start reduction.");
    let mut modifications = vec![];
    LoopReducer::<
        _,
        ChainReducer<
            _,
            ParallelSmartReducer<
                _,
                ChainReducer<
                    _,
                    ParallelSmartReducer<
                        _,
                        Chain5Reducer<
                            _,
                            ParallelSmartReducer<
                                _,
                                ChainReducer<_, IdentityPlaceReducer, SimpleLoopAgglomeration>,
                            >,
                            IdentityTransitionReducer,
                            SmartReducer<
                                _,
                                SimpleChainReducer,
                                ChainReducer<_, IdentityPlaceReducer, R7Reducer>,
                                IdentityTransitionReducer,
                            >,
                            R7Reducer,
                            PseudoStart,
                        >,
                    >,
                    RLReducer,
                >,
            >,
            WeightSimplification,
        >,
    >::reduce(&mut net, &mut modifications);

    let now = SystemTime::now();
    info!(
        "Reduction done: {:?}. {} modifications",
        now.elapsed()?,
        modifications.len()
    );
    write_output(&net, &modifications, &matches)?;
    Ok(())
}

fn write_output(
    net: &Net,
    modifications: &[Modification],
    matches: &ArgMatches,
) -> Result<(), Box<dyn Error>> {
    info!("Start writing new network.");
    let now = SystemTime::now();
    match matches.value_of("OUTPUT") {
        None => {}
        Some(f) if f != "-" => {
            let mut file = File::create(f)?;
            if matches.is_present("EQUATIONS") {
                write_modifications(&modifications, &mut file, &net)?;
            }
            ExporterBuilder::new(file)
                .with_all_places(!matches.is_present("CLEAN"))
                .with_disconnected_transitions(matches.is_present("CLEAN"))
                .build()
                .export(&net.into())?;
            if matches.is_present("RENUMBER") {
                let net = match matches.value_of("INPUT") {
                    Some("-") | None => {
                        pnets_tina::Parser::new(BufReader::new(io::stdin())).parse()?
                    }
                    Some(f) => pnets_tina::Parser::new(BufReader::new(File::open(f)?)).parse()?,
                };

                ExporterBuilder::new(File::create(f.to_owned() + ".orig.net")?)
                    .with_all_places(!matches.is_present("CLEAN"))
                    .with_disconnected_transitions(matches.is_present("CLEAN"))
                    .build()
                    .export(&net)?;
            }
        }
        Some(_) => {
            if matches.is_present("EQUATIONS") {
                write_modifications(&modifications, &mut stdout(), &net)?;
            }
            ExporterBuilder::new(stdout())
                .with_all_places(!matches.is_present("CLEAN"))
                .with_disconnected_transitions(matches.is_present("CLEAN"))
                .build()
                .export(&net.into())?
        }
    }
    info!("Writing done: {:?}.", now.elapsed()?);
    Ok(())
}

fn write_modifications<Writer: Write>(
    modifications: &[Modification],
    writer: &mut Writer,
    net: &Net,
) -> Result<(), Box<dyn Error>> {
    for modification in modifications {
        match modification {
            Modification::Agglomeration(agg) => {
                writer.write_all(
                    format!(
                        "# A |- {}*{} = ",
                        agg.factor,
                        net.get_name_by_index(&NodeId::Place(agg.new_place))
                            .unwrap()
                    )
                    .as_ref(),
                )?;
                for &(pl, w) in &agg.deleted_places {
                    writer.write_all(
                        format!(
                            "{}*{} + ",
                            w,
                            net.get_name_by_index(&NodeId::Place(pl)).unwrap()
                        )
                        .as_ref(),
                    )?;
                }
                writer.write_all(format!("{}\n", agg.constant).as_ref())?;
            }
            Modification::Reduction(red) => {
                writer.write_all(b"# R |- ")?;
                writer.write_all(format!("{}", red.constant).as_ref())?;
                for (pl, w) in &red.equals_to {
                    writer.write_all(b" + ")?;
                    writer.write_all(
                        format!(
                            "{}*{}",
                            w,
                            net.get_name_by_index(&NodeId::Place(*pl)).unwrap()
                        )
                        .as_ref(),
                    )?;
                }
                writer.write_all(b" = ")?;
                let mut first = true;
                for (pl, w) in &red.deleted_places {
                    if first {
                        first = false;
                    } else {
                        writer.write_all(b" + ")?;
                    }
                    writer.write_all(
                        format!(
                            "{}*{}",
                            w,
                            net.get_name_by_index(&NodeId::Place(*pl)).unwrap()
                        )
                        .as_ref(),
                    )?;
                }
                writer.write_all(b"\n")?;
            }
            Modification::TransitionElimination(_) => {}
            Modification::InequalityReduction(ine) => {
                writer.write_all(b"# I |- ")?;
                let mut first = true;
                for (pl, w) in &ine.deleted_places {
                    if first {
                        first = false;
                    } else {
                        writer.write_all(b" + ")?;
                    }
                    writer.write_all(
                        format!(
                            "{}*{}",
                            w,
                            net.get_name_by_index(&NodeId::Place(*pl)).unwrap()
                        )
                        .as_ref(),
                    )?;
                }
                writer.write_all(b" <= ")?;
                for (pl, w) in &ine.kept_places {
                    writer.write_all(
                        format!(
                            "{}*{} + ",
                            w,
                            net.get_name_by_index(&NodeId::Place(*pl)).unwrap()
                        )
                        .as_ref(),
                    )?;
                }
                writer.write_all(format!("{}\n", ine.constant).as_ref())?;
            }
        }
    }
    Ok(())
}
