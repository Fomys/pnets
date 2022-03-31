use clap::Parser;
use log::info;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::time::SystemTime;

use pnets::standard::Net;
use pnets::NodeId;
use pnets_pnml::ptnet::Ptnet;
use pnets_shrink::modifications::Modification;
use pnets_shrink::reducers::standard::{
    IdentityPlaceReducer, IdentityTransitionReducer, ParallelPlaceReducer,
    ParallelTransitionReducer, PseudoStart, RLReducer, SimpleChainReducer, SimpleLoopAgglomeration,
    SourceSinkReducer, WeightSimplification,
};
use pnets_shrink::reducers::{ChainReducer, IdentityReducer, LoopReducer, Reduce, SmartReducer};
use pnets_tina::ExporterBuilder;

#[derive(clap::ArgEnum, Clone, Copy)]
enum Format {
    PNML,
    Net,
    Guess,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File to read ("-" for stdin)
    #[clap(short, long, default_value = "-")]
    input: String,
    /// File to write ("-" for stdout)
    #[clap(short, long, default_value = "-")]
    output: String,
    /// Format for the input
    #[clap(short, long, default_value_t=Format::Guess)]
    #[clap(arg_enum)]
    format: Format,
    /// Print equations
    #[clap(short, long)]
    equations: bool,
    /// Remove all disconnected transitions and useless places from the net after reductions
    #[clap(long)]
    clean: bool,
    #[clap(long)]
    redundant: bool,
    #[clap(long)]
    compact: bool,
    #[clap(long)]
    extra: bool,
    #[clap(long, default_value_t=u64::MAX)]
    max_iter: u64,
}

fn new_redundant_compact_extra(
    args: &Args,
) -> LoopReducer<
    Net,
    ChainReducer<
        Net,
        SmartReducer<
            Net,
            ChainReducer<
                Net,
                SmartReducer<
                    Net,
                    ChainReducer<Net, IdentityPlaceReducer, SimpleLoopAgglomeration>,
                    ParallelPlaceReducer,
                    ParallelTransitionReducer,
                >,
                ChainReducer<
                    Net,
                    IdentityTransitionReducer,
                    ChainReducer<
                        Net,
                        SmartReducer<
                            Net,
                            SimpleChainReducer,
                            ChainReducer<Net, IdentityPlaceReducer, SourceSinkReducer>,
                            IdentityTransitionReducer,
                        >,
                        ChainReducer<Net, SourceSinkReducer, PseudoStart>,
                    >,
                >,
            >,
            ParallelPlaceReducer,
            ParallelTransitionReducer,
        >,
        ChainReducer<
            Net,
            RLReducer,
            ChainReducer<
                Net,
                WeightSimplification,
                ChainReducer<Net, ParallelPlaceReducer, ParallelTransitionReducer>,
            >,
        >,
    >,
> {
    LoopReducer::new(
        ChainReducer::new5(
            SmartReducer::new_parallel_smart_reducer(ChainReducer::new5(
                SmartReducer::new_parallel_smart_reducer(ChainReducer::new2(
                    IdentityPlaceReducer::new(),
                    SimpleLoopAgglomeration::new(),
                )),
                IdentityTransitionReducer::new(),
                SmartReducer::new(
                    SimpleChainReducer::new(),
                    ChainReducer::new2(IdentityPlaceReducer::new(), SourceSinkReducer::new()),
                    IdentityTransitionReducer::new(),
                ),
                SourceSinkReducer::new(),
                PseudoStart::new(),
            )),
            RLReducer::new(),
            WeightSimplification::new(),
            ParallelPlaceReducer::new(),
            ParallelTransitionReducer::new(),
        ),
        args.max_iter,
    )
}

fn new_redundant_compact(
    args: &Args,
) -> LoopReducer<
    Net,
    ChainReducer<
        Net,
        SmartReducer<
            Net,
            ChainReducer<Net, IdentityPlaceReducer, SimpleLoopAgglomeration>,
            ParallelPlaceReducer,
            ParallelTransitionReducer,
        >,
        ChainReducer<
            Net,
            SmartReducer<
                Net,
                SimpleChainReducer,
                ChainReducer<Net, IdentityPlaceReducer, SourceSinkReducer>,
                IdentityTransitionReducer,
            >,
            ChainReducer<
                Net,
                SourceSinkReducer,
                ChainReducer<Net, ParallelPlaceReducer, ParallelTransitionReducer>,
            >,
        >,
    >,
> {
    LoopReducer::new(
        ChainReducer::new5(
            SmartReducer::new_parallel_smart_reducer(ChainReducer::new2(
                IdentityPlaceReducer::new(),
                SimpleLoopAgglomeration::new(),
            )),
            SmartReducer::new(
                SimpleChainReducer::new(),
                ChainReducer::new2(IdentityPlaceReducer::new(), SourceSinkReducer::new()),
                IdentityTransitionReducer::new(),
            ),
            SourceSinkReducer::new(),
            ParallelPlaceReducer::new(),
            ParallelTransitionReducer::new(),
        ),
        args.max_iter,
    )
}

fn new_redundant_extra(
    args: &Args,
) -> LoopReducer<
    Net,
    ChainReducer<
        Net,
        SmartReducer<
            Net,
            ChainReducer<
                Net,
                SmartReducer<
                    Net,
                    IdentityPlaceReducer,
                    ParallelPlaceReducer,
                    ParallelTransitionReducer,
                >,
                ChainReducer<
                    Net,
                    IdentityTransitionReducer,
                    ChainReducer<
                        Net,
                        SourceSinkReducer,
                        ChainReducer<
                            Net,
                            PseudoStart,
                            ChainReducer<Net, RLReducer, WeightSimplification>,
                        >,
                    >,
                >,
            >,
            ParallelPlaceReducer,
            ParallelTransitionReducer,
        >,
        ChainReducer<Net, ParallelPlaceReducer, ParallelTransitionReducer>,
    >,
> {
    LoopReducer::new(
        ChainReducer::new3(
            SmartReducer::new_parallel_smart_reducer(ChainReducer::new6(
                SmartReducer::new_parallel_smart_reducer(IdentityPlaceReducer::new()),
                IdentityTransitionReducer::new(),
                SourceSinkReducer::new(),
                PseudoStart::new(),
                RLReducer::new(),
                WeightSimplification::new(),
            )),
            ParallelPlaceReducer::new(),
            ParallelTransitionReducer::new(),
        ),
        args.max_iter,
    )
}

fn new_redundant(
    args: &Args,
) -> LoopReducer<
    Net,
    ChainReducer<
        Net,
        SmartReducer<
            Net,
            ChainReducer<
                Net,
                SmartReducer<
                    Net,
                    IdentityPlaceReducer,
                    ParallelPlaceReducer,
                    ParallelTransitionReducer,
                >,
                ChainReducer<Net, IdentityTransitionReducer, SourceSinkReducer>,
            >,
            ParallelPlaceReducer,
            ParallelTransitionReducer,
        >,
        ChainReducer<Net, ParallelPlaceReducer, ParallelTransitionReducer>,
    >,
> {
    LoopReducer::new(
        ChainReducer::new3(
            SmartReducer::new_parallel_smart_reducer(ChainReducer::new3(
                SmartReducer::new_parallel_smart_reducer(IdentityPlaceReducer::new()),
                IdentityTransitionReducer::new(),
                SourceSinkReducer::new(),
            )),
            ParallelPlaceReducer::new(),
            ParallelTransitionReducer::new(),
        ),
        args.max_iter,
    )
}

fn new_compact_extra(
    args: &Args,
) -> LoopReducer<
    Net,
    ChainReducer<
        Net,
        SimpleLoopAgglomeration,
        ChainReducer<
            Net,
            SimpleChainReducer,
            ChainReducer<Net, PseudoStart, ChainReducer<Net, RLReducer, WeightSimplification>>,
        >,
    >,
> {
    LoopReducer::new(
        ChainReducer::new5(
            SimpleLoopAgglomeration::new(),
            SimpleChainReducer::new(),
            PseudoStart::new(),
            RLReducer::new(),
            WeightSimplification::new(),
        ),
        args.max_iter,
    )
}

fn new_compact(
    args: &Args,
) -> LoopReducer<Net, ChainReducer<Net, SimpleChainReducer, SimpleLoopAgglomeration>> {
    LoopReducer::new(
        ChainReducer::new2(SimpleChainReducer::new(), SimpleLoopAgglomeration::new()),
        args.max_iter,
    )
}

fn new_extra(
    args: &Args,
) -> LoopReducer<
    Net,
    ChainReducer<Net, PseudoStart, ChainReducer<Net, RLReducer, WeightSimplification>>,
> {
    LoopReducer::new(
        ChainReducer::new3(
            PseudoStart::new(),
            RLReducer::new(),
            WeightSimplification::new(),
        ),
        args.max_iter,
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    info!("Start parsing.");
    let now = SystemTime::now();
    let buf_reader: Box<dyn BufRead> = match (args.input.as_ref(), args.format) {
        ("-", Format::Guess) => {
            args.format = Format::Net;
            Box::new(BufReader::new(stdin()))
        }
        ("-", _) => Box::new(BufReader::new(stdin())),
        (s, Format::Guess) => {
            if s.ends_with("pnml") {
                args.format = Format::PNML;
            }
            Box::new(BufReader::new(File::open(s)?))
        }
        (s, _) => Box::new(BufReader::new(File::open(s)?)),
    };

    let mut net = match args.format {
        Format::Net => pnets_tina::Parser::new(buf_reader).parse()?.into(),
        Format::PNML => {
            let ptnet: Ptnet = quick_xml::de::from_reader(buf_reader)?;
            let mut nets: Vec<Net> = (&ptnet).try_into()?;
            nets.pop().unwrap()
        }
        Format::Guess => pnets_tina::Parser::new(buf_reader).parse()?.into(),
    };
    info!("Parsing done: {:?}", now.elapsed()?);
    info!("Start reduction.");
    let mut modifications = vec![];

    let reducer: Box<dyn Reduce<Net>> = match args {
        Args {
            redundant: true,
            compact: true,
            extra: true,
            ..
        } => Box::new(new_redundant_compact_extra(&args)),
        Args {
            redundant: true,
            compact: true,
            extra: false,
            ..
        } => Box::new(new_redundant_compact(&args)),
        Args {
            redundant: true,
            compact: false,
            extra: true,
            ..
        } => Box::new(new_redundant_extra(&args)),
        Args {
            redundant: true,
            compact: false,
            extra: false,
            ..
        } => Box::new(new_redundant(&args)),
        Args {
            redundant: false,
            compact: true,
            extra: true,
            ..
        } => Box::new(new_compact_extra(&args)),
        Args {
            redundant: false,
            compact: true,
            extra: false,
            ..
        } => Box::new(new_compact(&args)),
        Args {
            redundant: false,
            compact: false,
            extra: true,
            ..
        } => Box::new(new_extra(&args)),
        Args {
            redundant: false,
            compact: false,
            extra: false,
            ..
        } => Box::new(IdentityReducer::new()),
    };
    reducer.reduce(&mut net, &mut modifications);

    let now = SystemTime::now();
    info!(
        "Reduction done: {:?}. {} modifications",
        now.elapsed()?,
        modifications.len()
    );
    write_output(&net, &modifications, args)?;
    Ok(())
}

fn write_output(
    net: &Net,
    modifications: &[Modification],
    args: Args,
) -> Result<(), Box<dyn Error>> {
    info!("Start writing new net.");
    let now = SystemTime::now();
    let mut buf_writer: Box<dyn Write> = match args.output.as_ref() {
        "-" => Box::new(stdout()),
        s => Box::new(File::create(s)?),
    };

    if args.equations {
        write_modifications(&modifications, buf_writer.as_mut(), &net)?;
    }
    ExporterBuilder::new(buf_writer.as_mut())
        .with_all_places(!args.clean)
        .with_disconnected_transitions(args.clean)
        .build()
        .export(&net.into())?;
    info!("Writing done: {:?}.", now.elapsed()?);
    Ok(())
}

fn write_modifications(
    modifications: &[Modification],
    writer: &mut dyn Write,
    net: &Net,
) -> Result<(), Box<dyn Error>> {
    writer.write_all("# generated equations\n".as_ref())?;
    for modification in modifications {
        match modification {
            Modification::Agglomeration(agg) => {
                if agg.factor == 1 {
                    writer.write_all(
                        format!(
                            "# A |- {} = ",
                            net.get_name_by_index(&NodeId::Place(agg.new_place))
                                .unwrap()
                        )
                        .as_ref(),
                    )?;
                } else {
                    writer.write_all(
                        format!(
                            "# A |- {}*{} = ",
                            agg.factor,
                            net.get_name_by_index(&NodeId::Place(agg.new_place))
                                .unwrap()
                        )
                        .as_ref(),
                    )?;
                }
                for i in 0..agg.deleted_places.len() {
                    let (pl, w) = agg.deleted_places[i];
                    if w == 1 {
                        writer.write_all(
                            format!("{}", net.get_name_by_index(&NodeId::Place(pl)).unwrap())
                                .as_ref(),
                        )?;
                    } else {
                        writer.write_all(
                            format!(
                                "{}*{}",
                                w,
                                net.get_name_by_index(&NodeId::Place(pl)).unwrap()
                            )
                            .as_ref(),
                        )?;
                    }
                    if i + 1 != agg.deleted_places.len() {
                        writer.write_all(" + ".as_ref())?;
                    }
                }
                if agg.constant != 0 {
                    writer.write_all(format!("{}", agg.constant).as_ref())?;
                }
                writer.write_all("\n".as_ref())?;
            }
            Modification::Reduction(red) => {
                writer.write_all(b"# R |- ")?;
                for i in 0..red.deleted_places.len() {
                    let (pl, w) = red.deleted_places[i];
                    if w == 1 {
                        writer.write_all(
                            format!("{}", net.get_name_by_index(&NodeId::Place(pl)).unwrap())
                                .as_ref(),
                        )?;
                    } else {
                        writer.write_all(
                            format!(
                                "{}*{}",
                                w,
                                net.get_name_by_index(&NodeId::Place(pl)).unwrap()
                            )
                            .as_ref(),
                        )?;
                    }
                    if i + 1 != red.deleted_places.len() {
                        writer.write_all(b" + ")?;
                    }
                }
                writer.write_all(b" = ")?;
                let mut first = true;
                for &(pl, w) in &red.equals_to {
                    if first {
                        first = false;
                    } else {
                        writer.write_all(b" + ")?;
                    }
                    if w == 1 {
                        writer.write_all(
                            format!("{}", net.get_name_by_index(&NodeId::Place(pl)).unwrap())
                                .as_ref(),
                        )?;
                    } else {
                        writer.write_all(
                            format!(
                                "{}*{}",
                                w,
                                net.get_name_by_index(&NodeId::Place(pl)).unwrap()
                            )
                            .as_ref(),
                        )?;
                    }
                }
                if red.constant != 0 {
                    if !red.equals_to.is_empty() {
                        writer.write_all(" + ".as_ref())?;
                    }
                    writer.write_all(format!("{}", red.constant).as_ref())?;
                }
                writer.write_all(b"\n")?;
            }
            Modification::TransitionElimination(_) => {}
            Modification::InequalityReduction(ine) => {
                writer.write_all(b"# I |- ")?;
                let mut first = true;
                for &(pl, w) in &ine.deleted_places {
                    if first {
                        first = false;
                    } else {
                        writer.write_all(b" + ")?;
                    }
                    if w == 1 {
                        writer.write_all(
                            format!("{}", net.get_name_by_index(&NodeId::Place(pl)).unwrap())
                                .as_ref(),
                        )?;
                    } else {
                        writer.write_all(
                            format!(
                                "{}*{}",
                                w,
                                net.get_name_by_index(&NodeId::Place(pl)).unwrap()
                            )
                            .as_ref(),
                        )?;
                    }
                }
                writer.write_all(b" <= ")?;
                for i in 0..ine.kept_places.len() {
                    let (pl, w) = ine.kept_places[i];
                    if w == 1 {
                        writer.write_all(
                            format!("{} + ", net.get_name_by_index(&NodeId::Place(pl)).unwrap())
                                .as_ref(),
                        )?;
                    } else {
                        writer.write_all(
                            format!(
                                "{}*{} + ",
                                w,
                                net.get_name_by_index(&NodeId::Place(pl)).unwrap()
                            )
                            .as_ref(),
                        )?;
                    }
                }
                if ine.constant != 0 {
                    writer.write_all(format!("{}", ine.constant).as_ref())?;
                }
                writer.write_all("\n".as_ref())?;
            }
        }
    }
    writer.write_all("\n".as_ref())?;
    Ok(())
}
