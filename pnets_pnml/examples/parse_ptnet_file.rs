use pnets::standard;
use pnets_pnml::ptnet::Ptnet;
use serde_xml_rs::from_str;
use std::convert::TryInto;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ptnet: Ptnet = from_str(include_str!("HouseConstruction-002.pnml"))?;
    let nets: Vec<standard::Net> = (&ptnet).try_into()?;
    println!("{:?}", nets);
    let ptnet: Ptnet = (&nets).into();
    println!("{:?}", quick_xml::se::to_string(&ptnet));
    Ok(())
}
