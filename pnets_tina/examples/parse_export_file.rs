use pnets_tina::ExporterBuilder;
use std::error::Error;
use std::io::stdout;

fn main() -> Result<(), Box<dyn Error>> {
    let parser = pnets_tina::Parser::new(include_str!("sokoban_3.net").as_bytes());
    let net = parser.parse().unwrap();
    let mut writer = ExporterBuilder::new(stdout())
        .with_all_places(true)
        .with_disconnected_transitions(true)
        .build();
    writer.export(&net)?;
    Ok(())
}
