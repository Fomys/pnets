use pnets::Marking;

#[test]
fn test_get() {
    let mut marking: Marking<usize> = Default::default();
    marking.insert_or_add(2, 2);
    marking.insert_or_add(1, 3);
    marking.insert_or_add(3, 1);
    marking.insert_or_add(5, 5);

    assert_eq!(marking[1], 3);
    assert_eq!(marking[2], 2);
    assert_eq!(marking[3], 1);
    assert_eq!(marking[4], 0);
    assert_eq!(marking[5], 5);
    assert_eq!(marking[52], 0);
}

#[test]
fn test_insert_or_add() {
    let mut marking: Marking<usize> = Default::default();
    marking.insert_or_add(2, 2);
    marking.insert_or_add(1, 3);
    marking.insert_or_add(3, 1);
    marking.insert_or_add(2, 2);
    marking.insert_or_add(5, 5);
    marking.insert_or_add(4, 4);

    assert_eq!(marking[1], 3);
    assert_eq!(marking[2], 4);
    assert_eq!(marking[3], 1);
    assert_eq!(marking[4], 4);
    assert_eq!(marking[5], 5);
}

#[test]
fn test_insert_or_min() {
    let mut marking: Marking<usize> = Default::default();
    marking.insert_or_min(2, 2);
    marking.insert_or_min(1, 3);
    marking.insert_or_min(1, 1);
    marking.insert_or_min(2, 2);
    marking.insert_or_min(5, 5);
    marking.insert_or_min(4, 4);

    assert_eq!(marking[1], 1);
    assert_eq!(marking[2], 2);
    assert_eq!(marking[4], 4);
    assert_eq!(marking[5], 5);
}

#[test]
fn test_insert_or_max() {
    let mut marking: Marking<usize> = Default::default();
    marking.insert_or_max(2, 2);
    marking.insert_or_max(2, 3);
    marking.insert_or_max(2, 2);
    marking.insert_or_max(5, 5);
    marking.insert_or_max(4, 4);

    assert_eq!(marking[2], 3);
    assert_eq!(marking[4], 4);
    assert_eq!(marking[5], 5);
}
