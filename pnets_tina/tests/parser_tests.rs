use pnets::timed::{Bound, TimeRange};
use pnets::{NodeId, PlaceId, TransitionId};

#[test]
fn name_test() {
    let parser = pnets_tina::Parser::new("net Name".as_bytes());
    let net = parser.parse().unwrap();
    assert_eq!(net.name, "Name".to_string())
}

// Should panic because lb is not implemented
#[test]
fn label_test() {
    let parser = pnets_tina::Parser::new(
        "tr t0 p0 -> p1\nlb t0 {transition}\nlb p0 {place 0}\nlb p1 {place 1}".as_bytes(),
    );
    let net = parser.parse().unwrap();
    let t0 = net
        .get_index_by_name("t0")
        .unwrap()
        .as_transition()
        .unwrap();
    assert_eq!(net[t0].label, Some("transition".to_string()));
    let p0 = net.get_index_by_name("p0").unwrap().as_place().unwrap();
    assert_eq!(net[p0].label, Some("place 0".to_string()));
    let p1 = net.get_index_by_name("p1").unwrap().as_place().unwrap();
    assert_eq!(net[p1].label, Some("place 1".to_string()));
}

#[test]
fn transition_test() {
    let parser =
        pnets_tina::Parser::new("tr t0 : transition_label [1,5] p0*3 p2?4 p3?-2 -> p1".as_bytes());

    let net = parser.parse().unwrap();

    // Check if all places and transitions are created
    let t0: TransitionId = net
        .get_index_by_name(&"t0".to_string())
        .unwrap()
        .as_transition()
        .unwrap();
    let p0 = net
        .get_index_by_name(&"p0".to_string())
        .unwrap()
        .as_place()
        .unwrap();
    let p1: PlaceId = net
        .get_index_by_name(&"p1".to_string())
        .unwrap()
        .as_place()
        .unwrap();
    let p2: PlaceId = net
        .get_index_by_name(&"p2".to_string())
        .unwrap()
        .as_place()
        .unwrap();
    let p3: PlaceId = net
        .get_index_by_name(&"p3".to_string())
        .unwrap()
        .as_place()
        .unwrap();

    assert_eq!(
        net.get_name_by_index(&NodeId::Transition(t0)).unwrap(),
        "t0".to_string()
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(p0)).unwrap(),
        "p0".to_string()
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(p1)).unwrap(),
        "p1".to_string()
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(p2)).unwrap(),
        "p2".to_string()
    );
    assert_eq!(
        net.get_name_by_index(&NodeId::Place(p3)).unwrap(),
        "p3".to_string()
    );

    // Check transition information
    assert_eq!(
        net[t0].time,
        TimeRange {
            start: Bound::Closed(1),
            end: Bound::Closed(5),
        }
    );
    assert_eq!(net[t0].label, Some("transition_label".to_string()));
    assert_eq!(net[t0].consume[p0], 3);
    assert_eq!(net[t0].consume[p1], 0);
    assert_eq!(net[t0].consume[p2], 0);
    assert_eq!(net[t0].consume[p3], 0);

    assert_eq!(net[p0].consumed_by[t0], 3);
    assert_eq!(net[p1].consumed_by[t0], 0);
    assert_eq!(net[p2].consumed_by[t0], 0);
    assert_eq!(net[p3].consumed_by[t0], 0);

    assert_eq!(net[t0].produce[p0], 0);
    assert_eq!(net[t0].produce[p1], 1);
    assert_eq!(net[t0].produce[p2], 0);
    assert_eq!(net[t0].produce[p3], 0);

    assert_eq!(net[p0].produced_by[t0], 0);
    assert_eq!(net[p1].produced_by[t0], 1);
    assert_eq!(net[p2].produced_by[t0], 0);
    assert_eq!(net[p3].produced_by[t0], 0);

    assert_eq!(net[t0].conditions[p0], 0);
    assert_eq!(net[t0].conditions[p1], 0);
    assert_eq!(net[t0].conditions[p2], 4);
    assert_eq!(net[t0].conditions[p3], 0);

    assert_eq!(net[p0].condition_for[t0], 0);
    assert_eq!(net[p1].condition_for[t0], 0);
    assert_eq!(net[p2].condition_for[t0], 4);
    assert_eq!(net[p3].condition_for[t0], 0);

    assert_eq!(net[t0].inhibitors[p0], 0);
    assert_eq!(net[t0].inhibitors[p1], 0);
    assert_eq!(net[t0].inhibitors[p2], 0);
    assert_eq!(net[t0].inhibitors[p3], 2);

    assert_eq!(net[p0].inhibitor_for[t0], 0);
    assert_eq!(net[p1].inhibitor_for[t0], 0);
    assert_eq!(net[p2].inhibitor_for[t0], 0);
    assert_eq!(net[p3].inhibitor_for[t0], 2);
}

#[test]
#[should_panic]
fn name_test_invalid_identifier() {
    let parser = pnets_tina::Parser::new("net *".as_bytes());
    parser.parse().unwrap();
}

#[test]
fn place_test() {
    let parser = pnets_tina::Parser::new("pl p0 : label (43K) t0 -> t1 t2?1M t3?-2K".as_bytes());
    let net = parser.parse().unwrap();
    println!("{:?}", net);

    let p0 = net
        .get_index_by_name(&"p0".to_string())
        .unwrap()
        .as_place()
        .unwrap();
    let t0 = net
        .get_index_by_name(&"t0".to_string())
        .unwrap()
        .as_transition()
        .unwrap();
    let t1 = net
        .get_index_by_name(&"t1".to_string())
        .unwrap()
        .as_transition()
        .unwrap();
    let t2 = net
        .get_index_by_name(&"t2".to_string())
        .unwrap()
        .as_transition()
        .unwrap();
    let t3 = net
        .get_index_by_name(&"t3".to_string())
        .unwrap()
        .as_transition()
        .unwrap();

    assert_eq!(net.get_name_by_index(&t0.into()).unwrap(), "t0".to_string());
    assert_eq!(net.get_name_by_index(&t1.into()).unwrap(), "t1".to_string());
    assert_eq!(net.get_name_by_index(&t2.into()).unwrap(), "t2".to_string());
    assert_eq!(net.get_name_by_index(&t3.into()).unwrap(), "t3".to_string());
    assert_eq!(net.get_name_by_index(&p0.into()).unwrap(), "p0".to_string());

    assert_eq!(net[p0].label, Some("label".to_string()));
    assert_eq!(net[p0].initial, 43000);

    assert_eq!(net[t0].produce[p0], 1);
    assert_eq!(net[t1].produce[p0], 0);
    assert_eq!(net[t2].produce[p0], 0);
    assert_eq!(net[t3].produce[p0], 0);

    assert_eq!(net[p0].produced_by[t0], 1);
    assert_eq!(net[p0].produced_by[t1], 0);
    assert_eq!(net[p0].produced_by[t2], 0);
    assert_eq!(net[p0].produced_by[t3], 0);

    assert_eq!(net[t0].consume[p0], 0);
    assert_eq!(net[t1].consume[p0], 1);
    assert_eq!(net[t2].consume[p0], 0);
    assert_eq!(net[t3].consume[p0], 0);

    assert_eq!(net[p0].consumed_by[t0], 0);
    assert_eq!(net[p0].consumed_by[t1], 1);
    assert_eq!(net[p0].consumed_by[t2], 0);
    assert_eq!(net[p0].consumed_by[t3], 0);

    assert_eq!(net[t0].conditions[p0], 0);
    assert_eq!(net[t1].conditions[p0], 0);
    assert_eq!(net[t2].conditions[p0], 1_000_000);
    assert_eq!(net[t3].conditions[p0], 0);

    assert_eq!(net[p0].condition_for[t0], 0);
    assert_eq!(net[p0].condition_for[t1], 0);
    assert_eq!(net[p0].condition_for[t2], 1_000_000);
    assert_eq!(net[p0].condition_for[t3], 0);

    assert_eq!(net[t0].inhibitors[p0], 0);
    assert_eq!(net[t1].inhibitors[p0], 0);
    assert_eq!(net[t2].inhibitors[p0], 0);
    assert_eq!(net[t3].inhibitors[p0], 2000);

    assert_eq!(net[p0].inhibitor_for[t0], 0);
    assert_eq!(net[p0].inhibitor_for[t1], 0);
    assert_eq!(net[p0].inhibitor_for[t2], 0);
    assert_eq!(net[p0].inhibitor_for[t3], 2_000);
}

#[test]
fn note_test() {
    let parser = pnets_tina::Parser::new("nt note 0 {This is a note}".as_bytes());
    parser.parse().unwrap();
}

#[test]
fn priority_test() {
    let parser = pnets_tina::Parser::new("".as_bytes());
    parser.parse().unwrap();
}

#[test]
fn demo_test() {
    let parser = pnets_tina::Parser::new(include_str!("demo.net").as_bytes());
    parser.parse().unwrap();
}

#[test]
fn abp_test() {
    let parser = pnets_tina::Parser::new(include_str!("abp.net").as_bytes());
    parser.parse().unwrap();
}

#[test]
fn ifip_test() {
    let parser = pnets_tina::Parser::new(include_str!("ifip.net").as_bytes());
    parser.parse().unwrap();
}

#[test]
fn sokoban_3_test() {
    let parser = pnets_tina::Parser::new(include_str!("sokoban_3.net").as_bytes());
    parser.parse().unwrap();
}
