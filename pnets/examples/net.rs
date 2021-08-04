use pnets::{arc, standard, NodeId};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut net = standard::Net::default();
    // Create places and transitions
    let pl_0 = net.create_place();
    let pl_1 = net.create_place();
    let pl_2 = net.create_place();
    let pl_3 = net.create_place();
    let pl_4 = net.create_place();
    let pl_5 = net.create_place();
    let pl_6 = net.create_place();
    let tr_0 = net.create_transition();
    let tr_1 = net.create_transition();
    let tr_2 = net.create_transition();
    let tr_3 = net.create_transition();
    let tr_4 = net.create_transition();
    // Rename node (by default they have automatic name)
    net.rename_node(pl_0.into(), "p0")?;
    net.rename_node(pl_1.into(), "p1")?;
    net.rename_node(pl_2.into(), "p2")?;
    net.rename_node(pl_3.into(), "p3")?;
    net.rename_node(pl_4.into(), "p4")?;
    net.rename_node(pl_5.into(), "p5")?;
    net.rename_node(pl_6.into(), "p6")?;
    net.rename_node(tr_0.into(), "t0")?;
    net.rename_node(tr_1.into(), "t1")?;
    net.rename_node(tr_2.into(), "t2")?;
    net.rename_node(tr_3.into(), "t3")?;
    net.rename_node(tr_4.into(), "t4")?;

    // Set label for places and transitions
    net[tr_0].label = Some("a".to_string());
    net[tr_1].label = Some("τ".to_string());
    net[tr_3].label = Some("b".to_string());
    net[tr_4].label = Some("c".to_string());

    // Set initial values
    net[pl_0].initial = 5;
    net[pl_6].initial = 4;

    // Create arcs
    net.add_arc(arc::Kind::Consume(pl_6, tr_4, 1))?;
    net.add_arc(arc::Kind::Consume(pl_3, tr_4, 1))?;
    net.add_arc(arc::Kind::Consume(pl_3, tr_2, 1))?;
    net.add_arc(arc::Kind::Consume(pl_5, tr_3, 1))?;
    net.add_arc(arc::Kind::Consume(pl_4, tr_3, 1))?;
    net.add_arc(arc::Kind::Consume(pl_2, tr_3, 1))?;
    net.add_arc(arc::Kind::Consume(pl_1, tr_1, 1))?;
    net.add_arc(arc::Kind::Consume(pl_0, tr_0, 1))?;
    net.add_arc(arc::Kind::Produce(pl_3, tr_4, 1))?;
    net.add_arc(arc::Kind::Produce(pl_3, tr_0, 1))?;
    net.add_arc(arc::Kind::Produce(pl_4, tr_2, 1))?;
    net.add_arc(arc::Kind::Produce(pl_1, tr_0, 1))?;
    net.add_arc(arc::Kind::Produce(pl_0, tr_0, 1))?;

    // Get node id by its name
    println!(
        "Place with name p0 has id {:?}",
        net.get_index_by_name("p0")
    );
    // Get node name
    println!(
        "Transition with id {:?} has name {}",
        NodeId::Transition(tr_0),
        net.get_name_by_index(&tr_0.into()).unwrap()
    );
    Ok(())
}
