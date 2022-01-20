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

#[test]
fn test_dual_iterator_rand() {
    let mut marking_1: Marking<usize> = Default::default();
    let mut marking_2: Marking<usize> = Default::default();
    for _ in 0..fastrand::usize(0..1000) {
        marking_1.insert_or_max(
            fastrand::usize(0..usize::MAX),
            fastrand::usize(0..usize::MAX),
        );
    }
    for _ in 0..fastrand::usize(0..1000) {
        marking_2.insert_or_max(
            fastrand::usize(0..usize::MAX),
            fastrand::usize(0..usize::MAX),
        );
    }
    for (idx, v1, v2) in marking_1.iter_with(&marking_2) {
        assert_eq!(v1, marking_1[idx]);
        assert_eq!(v2, marking_2[idx]);
    }
}

#[test]
fn test_dual_iterator_left() {
    let mut marking_1: Marking<usize> = Default::default();
    let marking_2: Marking<usize> = Default::default();
    marking_1.insert_or_max(0, 1);
    marking_1.insert_or_max(1, 1);
    marking_1.insert_or_max(2, 1);
    for (idx, v1, v2) in marking_1.iter_with(&marking_2) {
        assert_eq!(v1, marking_1[idx]);
        assert_eq!(v2, marking_2[idx]);
    }
}

#[test]
fn test_dual_iterator_right() {
    let marking_1: Marking<usize> = Default::default();
    let mut marking_2: Marking<usize> = Default::default();
    marking_2.insert_or_max(0, 1);
    marking_2.insert_or_max(1, 1);
    marking_2.insert_or_max(2, 1);
    for (idx, v1, v2) in marking_1.iter_with(&marking_2) {
        assert_eq!(v1, marking_1[idx]);
        assert_eq!(v2, marking_2[idx]);
    }
}
