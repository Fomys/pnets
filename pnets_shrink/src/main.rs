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
    IdentityPlaceReducer, IdentityTransitionReducer, InvariantReducer, ParallelPlaceReducer,
    ParallelSmartReducer, ParallelTransitionReducer, PseudoStart, RLReducer, SimpleChainReducer,
    SimpleLoopAgglomeration, SourceSinkReducer, WeightSimplification,
};
use pnets_shrink::reducers::{
    Chain3Reducer, Chain4Reducer, Chain5Reducer, Chain6Reducer, Chain7Reducer, ChainReducer,
    LoopReducer, Reduce, SmartReducer,
};
use pnets_tina::ExporterBuilder;

#[derive(clap::ArgEnum, Clone, Copy)]
enum Format {
    PNML,
    Net,
    Guess,
}

type AllReductions<N> = LoopReducer<
    N,
    Chain6Reducer<
        N,
        ParallelSmartReducer<
            N,
            Chain5Reducer<
                N,
                ParallelSmartReducer<
                    N,
                    ChainReducer<N, IdentityPlaceReducer, SimpleLoopAgglomeration>,
                >,
                IdentityTransitionReducer,
                SmartReducer<
                    N,
                    SimpleChainReducer,
                    ChainReducer<N, IdentityPlaceReducer, SourceSinkReducer>,
                    IdentityTransitionReducer,
                >,
                SourceSinkReducer,
                PseudoStart,
            >,
        >,
        RLReducer,
        WeightSimplification,
        ParallelPlaceReducer,
        ParallelTransitionReducer,
        InvariantReducer,
    >,
>;

type RedundantExtraCompactReducer<N> = LoopReducer<
    N,
    Chain5Reducer<
        N,
        ParallelSmartReducer<
            N,
            Chain5Reducer<
                N,
                ParallelSmartReducer<
                    N,
                    ChainReducer<N, IdentityPlaceReducer, SimpleLoopAgglomeration>,
                >,
                IdentityTransitionReducer,
                SmartReducer<
                    N,
                    SimpleChainReducer,
                    ChainReducer<N, IdentityPlaceReducer, SourceSinkReducer>,
                    IdentityTransitionReducer,
                >,
                SourceSinkReducer,
                PseudoStart,
            >,
        >,
        RLReducer,
        WeightSimplification,
        ParallelPlaceReducer,
        ParallelTransitionReducer,
    >,
>;

type ExtraReductions<N> =
    LoopReducer<N, Chain3Reducer<N, PseudoStart, RLReducer, WeightSimplification>>;

type ExtraStructReductions<N> = LoopReducer<
    N,
    Chain4Reducer<N, PseudoStart, RLReducer, WeightSimplification, InvariantReducer>,
>;

type RedundantReductions<N> = LoopReducer<
    N,
    Chain3Reducer<
        N,
        ParallelSmartReducer<
            N,
            Chain3Reducer<
                N,
                ParallelSmartReducer<N, IdentityPlaceReducer>,
                IdentityTransitionReducer,
                SourceSinkReducer,
            >,
        >,
        ParallelPlaceReducer,
        ParallelTransitionReducer,
    >,
>;

type RedundantStructReductions<N> = LoopReducer<
    N,
    Chain3Reducer<
        N,
        ParallelSmartReducer<
            N,
            Chain4Reducer<
                N,
                ParallelSmartReducer<N, IdentityPlaceReducer>,
                IdentityTransitionReducer,
                SourceSinkReducer,
                InvariantReducer,
            >,
        >,
        ParallelPlaceReducer,
        ParallelTransitionReducer,
    >,
>;

type RedundantExtraReductions<N> = LoopReducer<
    N,
    Chain3Reducer<
        N,
        ParallelSmartReducer<
            N,
            Chain6Reducer<
                N,
                ParallelSmartReducer<N, IdentityPlaceReducer>,
                IdentityTransitionReducer,
                SourceSinkReducer,
                PseudoStart,
                RLReducer,
                WeightSimplification,
            >,
        >,
        ParallelPlaceReducer,
        ParallelTransitionReducer,
    >,
>;

type RedundantStructExtraReductions<N> = LoopReducer<
    N,
    Chain3Reducer<
        N,
        ParallelSmartReducer<
            N,
            Chain7Reducer<
                N,
                ParallelSmartReducer<N, IdentityPlaceReducer>,
                IdentityTransitionReducer,
                SourceSinkReducer,
                PseudoStart,
                RLReducer,
                WeightSimplification,
                InvariantReducer,
            >,
        >,
        ParallelPlaceReducer,
        ParallelTransitionReducer,
    >,
>;

type CompactReductions<N> =
    LoopReducer<N, ChainReducer<N, SimpleLoopAgglomeration, SimpleChainReducer>>;
type CompactStructReductions<N> =
    LoopReducer<N, Chain3Reducer<N, SimpleLoopAgglomeration, SimpleChainReducer, InvariantReducer>>;

type CompactExtraReductions<N> = LoopReducer<
    N,
    Chain5Reducer<
        N,
        SimpleLoopAgglomeration,
        SimpleChainReducer,
        PseudoStart,
        RLReducer,
        WeightSimplification,
    >,
>;

type CompactStructExtraReductions<N> = LoopReducer<
    N,
    Chain6Reducer<
        N,
        SimpleLoopAgglomeration,
        SimpleChainReducer,
        PseudoStart,
        RLReducer,
        WeightSimplification,
        InvariantReducer,
    >,
>;

type CompactRedundantReductions<N> = LoopReducer<
    N,
    Chain5Reducer<
        N,
        ParallelSmartReducer<N, ChainReducer<N, IdentityPlaceReducer, SimpleLoopAgglomeration>>,
        SmartReducer<
            N,
            SimpleChainReducer,
            ChainReducer<N, IdentityPlaceReducer, SourceSinkReducer>,
            IdentityTransitionReducer,
        >,
        SourceSinkReducer,
        ParallelPlaceReducer, //
        ParallelTransitionReducer,
    >,
>;

type CompactStructRedundantReductions<N> = LoopReducer<
    N,
    Chain7Reducer<
        N,
        ParallelSmartReducer<N, ChainReducer<N, IdentityPlaceReducer, SimpleLoopAgglomeration>>,
        IdentityTransitionReducer,
        SmartReducer<
            N,
            SimpleChainReducer,
            ChainReducer<N, IdentityPlaceReducer, SourceSinkReducer>,
            IdentityTransitionReducer,
        >,
        SourceSinkReducer,
        ParallelPlaceReducer,
        ParallelTransitionReducer,
        InvariantReducer,
    >,
>;

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
    #[clap(long, name = "struct")]
    struct_: bool,
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
    match (args.redundant, args.compact, args.extra, args.struct_) {
        (true, true, true, true) => AllReductions::<_>::reduce(&mut net, &mut modifications),
        (false, true, true, true) => {
            CompactStructExtraReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (true, false, true, true) => {
            RedundantStructExtraReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (false, false, true, true) => {
            ExtraStructReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (true, true, false, true) => {
            CompactStructRedundantReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (false, true, false, true) => {
            CompactStructReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (true, false, false, true) => {
            RedundantStructReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (false, false, false, true) => InvariantReducer::reduce(&mut net, &mut modifications),
        (true, true, true, false) => {
            RedundantExtraCompactReducer::<_>::reduce(&mut net, &mut modifications)
        }
        (false, true, true, false) => {
            CompactExtraReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (true, false, true, false) => {
            RedundantExtraReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (false, false, true, false) => ExtraReductions::<_>::reduce(&mut net, &mut modifications),
        (true, true, false, false) => {
            CompactRedundantReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (false, true, false, false) => CompactReductions::<_>::reduce(&mut net, &mut modifications),
        (true, false, false, false) => {
            RedundantReductions::<_>::reduce(&mut net, &mut modifications)
        }
        (false, false, false, false) => {}
    }

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
        println!("{:?}", modification);
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
