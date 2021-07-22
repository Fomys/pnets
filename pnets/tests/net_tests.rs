use pnets::timed::Net;

#[test]
fn create_place() {
    let mut net = Net::default();
    let i = net.create_place();
    assert_eq!(net.places.len(), 1);
    assert_eq!(net[i].name, "".to_string());
    assert_eq!(net[i].label, "".to_string());
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
