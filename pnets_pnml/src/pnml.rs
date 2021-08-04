use crate::core::{EdgeGraphics, Name, Net, NodeGraphics, Page, SimpleText};
use pnets::arc::Kind;
use pnets::{NetError, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;

pub type Pnml = crate::core::Pnml<crate::core::Net<Place, Transition, Arc>>;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "place")]
pub struct Place {
    pub id: String,
    pub name: Option<Name>,
    pub graphics: Option<NodeGraphics>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "arc")]
pub struct Arc {
    pub id: String,
    pub source: String,
    pub target: String,
    pub name: Option<Name>,
    pub graphics: Option<EdgeGraphics>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "transition")]
pub struct Transition {
    pub id: String,
    pub name: Option<Name>,
    pub graphics: Option<NodeGraphics>,
}

impl Page<Place, Transition, Arc> {
    fn concat_places_transitions_to_net(
        &self,
        net: &mut pnets::standard::Net,
        reference_map: &mut HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        for page in &self.pages {
            page.concat_places_transitions_to_net(net, reference_map)?;
        }

        for place in &self.places {
            let pl = net.create_place();
            net.rename_node(NodeId::Place(pl), &place.id)?;
        }

        for transition in &self.transitions {
            let tr = net.create_transition();
            net.rename_node(NodeId::Transition(tr), &transition.id)?;
        }

        for pl_ref in &self.place_references {
            reference_map.insert(pl_ref.id.clone(), pl_ref.ref_.clone());
        }

        for tr_ref in &self.transition_references {
            reference_map.insert(tr_ref.id.clone(), tr_ref.ref_.clone());
        }
        Ok(())
    }

    fn concat_arcs_to_net(
        &self,
        net: &mut pnets::standard::Net,
        reference_map: &HashMap<String, String>,
    ) -> Result<(), NetError> {
        for arc in &self.arcs {
            let source = {
                let mut id = None;
                let mut source = arc.source.clone();
                while id.is_none() {
                    id = match net.get_index_by_name(&arc.source) {
                        None => {
                            source = match reference_map.get(&source) {
                                None => return Err(NetError::UnknownIdentifier(source)),
                                Some(source) => source.clone(),
                            };
                            None
                        }
                        Some(id) => Some(id),
                    }
                }
                id
            };
            let target = {
                let mut id = None;
                let mut target = arc.target.clone();
                while id.is_none() {
                    id = match net.get_index_by_name(&arc.target) {
                        None => {
                            target = match reference_map.get(&target) {
                                None => return Err(NetError::UnknownIdentifier(target)),
                                Some(target) => target.clone(),
                            };
                            None
                        }
                        Some(id) => Some(id),
                    }
                }
                id
            };
            match (source, target) {
                (Some(NodeId::Place(pl)), Some(NodeId::Transition(tr))) => {
                    net.add_arc(Kind::Consume(pl, tr, 1))?;
                }
                (Some(NodeId::Transition(tr)), Some(NodeId::Place(pl))) => {
                    net.add_arc(Kind::Produce(pl, tr, 1))?;
                }
                _ => return Err(NetError::InvalidArc),
            }
        }
        Ok(())
    }
}

impl TryInto<pnets::standard::Net> for &crate::core::Net<Place, Transition, Arc> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<pnets::standard::Net, Self::Error> {
        let mut net = pnets::standard::Net::default();
        if let Some(name) = &self.name {
            net.name = name.text.text.clone();
        }
        let mut reference_map: HashMap<String, String> = HashMap::default();
        for page in &self.pages {
            page.concat_places_transitions_to_net(&mut net, &mut reference_map)?;
        }
        for page in &self.pages {
            page.concat_arcs_to_net(&mut net, &reference_map)?;
        }
        Ok(net)
    }
}

impl TryInto<Vec<pnets::standard::Net>> for &Pnml {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<pnets::standard::Net>, Self::Error> {
        let mut nets = vec![];

        for net in &self.nets {
            nets.push(net.try_into()?);
        }
        Ok(nets)
    }
}

impl From<&Vec<pnets::standard::Net>> for Pnml {
    fn from(nets: &Vec<pnets::standard::Net>) -> Self {
        let mut pnml = Pnml {
            xmlns: "http://www.pnml.org/version-2009/grammar/pnml".to_string(),
            nets: vec![],
        };
        let mut net_count = 0;
        let mut arc_count = 0;
        for net in nets {
            net_count += 1;
            let mut new_net = Net::<Place, Transition, Arc> {
                type_: "http://www.pnml.org/version-2009/grammar/pnmlcoremodel".to_string(),
                ..Net::default()
            };
            if net.name.is_empty() {
                new_net.id = format!("net-auto-{}", net_count);
            } else {
                new_net.id = net.name.clone();
                new_net.name = Some(Name {
                    text: SimpleText {
                        text: net.name.clone(),
                    },
                    graphics: None,
                })
            }

            let mut page = Page::<Place, Transition, Arc> {
                id: format!("{}-main-page", new_net.id,),
                ..Page::default()
            };

            for (pl, place) in net.places.iter_enumerated() {
                let mut new_place = Place {
                    id: net.get_name_by_index(&pl.into()).unwrap(),
                    ..Place::default()
                };
                if place.label.is_some() {
                    {
                        new_place.name = Some(Name {
                            text: SimpleText {
                                text: place.label.as_ref().unwrap().clone(),
                            },
                            graphics: None,
                        });
                    }
                }
                page.places.push(new_place);
            }
            for (tr, transition) in net.transitions.iter_enumerated() {
                let mut new_transition = Transition {
                    id: net.get_name_by_index(&tr.into()).unwrap(),
                    ..Transition::default()
                };
                if transition.label.is_some() {
                    {
                        new_transition.name = Some(Name {
                            text: SimpleText {
                                text: transition.label.as_ref().unwrap().clone(),
                            },
                            graphics: None,
                        });
                    }
                }
                for &(pl, _) in transition.consume.iter() {
                    arc_count += 1;
                    page.arcs.push(Arc {
                        id: format!(
                            "{}-arcs-{}-{}-{}",
                            new_net.id,
                            net.get_name_by_index(&(pl.into())).unwrap(),
                            net.get_name_by_index(&tr.into()).unwrap(),
                            arc_count
                        ),
                        source: net.get_name_by_index(&(pl.into())).unwrap(),
                        target: net.get_name_by_index(&tr.into()).unwrap(),
                        name: None,
                        graphics: None,
                    })
                }
                for &(pl, _) in transition.produce.iter() {
                    arc_count += 1;
                    page.arcs.push(Arc {
                        id: format!(
                            "{}-arcs-{}-{}-{}",
                            new_net.id,
                            net.get_name_by_index(&tr.into()).unwrap(),
                            net.get_name_by_index(&(pl.into())).unwrap(),
                            arc_count
                        ),
                        target: net.get_name_by_index(&(pl.into())).unwrap(),
                        source: net.get_name_by_index(&tr.into()).unwrap(),
                        name: None,
                        graphics: None,
                    })
                }
                page.transitions.push(new_transition);
            }
            new_net.pages.push(page);
            pnml.nets.push(new_net);
        }
        pnml
    }
}
