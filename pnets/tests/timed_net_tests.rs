use pnets::arc::Kind;
use pnets::timed::Net;
use pnets::{NetError, NodeId, PlaceId, TransitionId};

#[test]
fn rename_node() {
    let mut net = Net::default();
    let tr = net.create_transition();
    let pl = net.create_place();
    assert_eq!(
        net.get_name_by_index(&NodeId::Transition(tr)),
        Some("0".to_string())
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(pl)),
        Some("1".to_string())
    );
    assert_eq!(net.rename_node(NodeId::Place(pl), "pl"), Ok(()));
    assert_eq!(net.rename_node(NodeId::Transition(tr), "tr"), Ok(()));
    assert_eq!(
        net.get_name_by_index(&NodeId::Transition(tr)),
        Some("tr".to_string())
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(pl)),
        Some("pl".to_string())
    );
    assert_eq!(
        net.rename_node(NodeId::Place(pl), "tr"),
        Err(NetError::DuplicatedName("tr".to_string()))
    );
    assert_eq!(
        net.rename_node(NodeId::Transition(tr), "pl"),
        Err(NetError::DuplicatedName("pl".to_string()))
    );
}

#[test]
fn transition() {
    let mut net = Net::default();
    let tr = net.create_transition();
    let pl = net.create_place();
    net.add_arc(Kind::Consume(pl, tr, 1)).unwrap();
    net.add_arc(Kind::Produce(pl, tr, 1)).unwrap();
    net.add_arc(Kind::Inhibitor(pl, tr, 1)).unwrap();
    net.add_arc(Kind::Test(pl, tr, 1)).unwrap();
    assert_eq!(net[tr].id(), tr);
    assert_eq!(net[tr].is_disconnected(), false);
    net.delete_transition(tr);
    assert_eq!(net[tr].is_disconnected(), true);
    assert_eq!(net[tr].id(), tr);
}

#[test]
fn places() {
    let mut net = Net::default();
    let tr = net.create_transition();
    let pl = net.create_place();
    net.add_arc(Kind::Consume(pl, tr, 1)).unwrap();
    net.add_arc(Kind::Produce(pl, tr, 1)).unwrap();
    net.add_arc(Kind::Inhibitor(pl, tr, 1)).unwrap();
    net.add_arc(Kind::Test(pl, tr, 1)).unwrap();
    assert_eq!(net[pl].id(), pl);
    assert_eq!(net[pl].is_disconnected(), false);
    net.delete_place(pl);
    assert_eq!(net[pl].is_disconnected(), true);
    assert_eq!(net[pl].id(), pl);
}

#[test]
fn create_transition_with_used_name() {
    let mut net = Net::default();
    let tr_1 = net.create_transition();
    net.rename_node(NodeId::Transition(tr_1), "tr_1").unwrap();
    let tr_2 = net.create_transition();
    assert_eq!(net.transitions.len(), 2);
    assert_eq!(
        net.get_name_by_index(&NodeId::Transition(tr_1)).unwrap(),
        "tr_1".to_string()
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Transition(tr_2)).unwrap(),
        "a1".to_string()
    );
    assert_eq!(net[tr_1].label, None);
    assert_eq!(net[tr_2].label, None);
}

#[test]
fn create_place_with_used_name() {
    let mut net = Net::default();
    let pl_1 = net.create_place();
    net.rename_node(NodeId::Place(pl_1), "pl_1").unwrap();
    let pl_2 = net.create_place();
    assert_eq!(net.places.len(), 2);
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(pl_1)).unwrap(),
        "pl_1".to_string()
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(pl_2)).unwrap(),
        "a1".to_string()
    );
    assert_eq!(net[pl_1].label, None);
    assert_eq!(net[pl_2].label, None);
}

#[test]
fn create_transition() {
    let mut net = Net::default();
    let i = net.create_transition();
    assert_eq!(net.transitions.len(), 1);
    assert_eq!(
        net.get_name_by_index(&NodeId::Transition(i)).unwrap(),
        "0".to_string()
    )
}

#[test]
fn create_place() {
    let mut net = Net::default();
    let i = net.create_place();
    assert_eq!(net.places.len(), 1);
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(i)).unwrap(),
        "0".to_string()
    );
    assert_eq!(net[i].label, None);
}

#[test]
fn update_priorities_test() {
    let mut net = Net::default();
    let t0 = net.create_transition();
    let t1 = net.create_transition();
    let t2 = net.create_transition();
    let t3 = net.create_transition();

    net.add_priority(t0, t1);
    net.add_priority(t0, t2);
    net.add_priority(t1, t2);
    net.add_priority(t2, t3);

    assert!(net.update_priorities().is_ok());
    assert_eq!(net[t0].priorities, vec![t1, t2, t3]);
    assert_eq!(net[t1].priorities, vec![t2, t3]);
    assert_eq!(net[t2].priorities, vec![t3]);
    assert_eq!(net[t3].priorities, vec![]);
}

#[test]
fn node_transition_cast_test() {
    let tr = TransitionId::from(0);
    assert_eq!(
        NodeId::Transition(tr).as_transition(),
        Some(TransitionId::from(0))
    );
    assert_eq!(NodeId::Transition(tr).as_place(), None);
}

#[test]
fn node_place_cast_test() {
    let pl = PlaceId::from(0);
    assert_eq!(NodeId::Place(pl).as_transition(), None);
    assert_eq!(NodeId::Place(pl).as_place(), Some(PlaceId::from(0)));
}
