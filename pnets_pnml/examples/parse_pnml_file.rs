use pnets::standard;
use pnets_pnml::pnml::Pnml;
use std::convert::TryInto;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let pnml: Pnml = quick_xml::de::from_str(include_str!("HouseConstruction-002.pnml"))?;
    let nets: Vec<standard::Net> = (&pnml).try_into()?;
    let pnml: Pnml = (&nets).into();
    println!("{:?}", quick_xml::se::to_string(&pnml));
    Ok(())
}
