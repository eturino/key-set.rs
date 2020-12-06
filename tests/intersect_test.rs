use key_set::KeySet;

#[test]
fn test_all() {
    let ks = KeySet::All as KeySet<i32>;

    assert_eq!(ks.intersect(&KeySet::All), KeySet::All);
    assert_eq!(ks.intersect(&KeySet::None), KeySet::None);
    assert_eq!(
        ks.intersect(&KeySet::Some(vec![1, 2, 3])),
        KeySet::Some(vec![1, 2, 3])
    );
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
}

#[test]
fn test_none() {
    let ks = KeySet::None as KeySet<i32>;

    assert_eq!(ks.intersect(&KeySet::All), KeySet::None);
    assert_eq!(ks.intersect(&KeySet::None), KeySet::None);
    assert_eq!(ks.intersect(&KeySet::Some(vec![1, 2, 3])), KeySet::None);
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::None
    );
}

#[test]
fn test_some() {
    let ks = KeySet::Some(vec![1, 2, 3]);

    assert_eq!(ks.intersect(&KeySet::All), KeySet::Some(vec![1, 2, 3]));
    assert_eq!(ks.intersect(&KeySet::None), KeySet::None);

    assert_eq!(
        ks.intersect(&KeySet::Some(vec![1, 2])),
        KeySet::Some(vec![1, 2])
    );
    assert_eq!(
        ks.intersect(&KeySet::Some(vec![1, 2, 3])),
        KeySet::Some(vec![1, 2, 3])
    );
    assert_eq!(
        ks.intersect(&KeySet::Some(vec![1, 3, 4, 5])),
        KeySet::Some(vec![1, 3])
    );
    assert_eq!(ks.intersect(&KeySet::Some(vec![4, 5])), KeySet::None);

    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 2])),
        KeySet::Some(vec![3])
    );
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::None
    );
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 3, 4, 5])),
        KeySet::Some(vec![2])
    );
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![4, 5])),
        KeySet::Some(vec![1, 2, 3])
    );
}

#[test]
fn test_all_except_some() {
    let ks = KeySet::AllExceptSome(vec![1, 2, 3]);

    assert_eq!(
        ks.intersect(&KeySet::All),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(ks.intersect(&KeySet::None), KeySet::None);

    assert_eq!(ks.intersect(&KeySet::Some(vec![1, 2])), KeySet::None);
    assert_eq!(ks.intersect(&KeySet::Some(vec![1, 2, 3])), KeySet::None);
    assert_eq!(
        ks.intersect(&KeySet::Some(vec![1, 3, 4, 5])),
        KeySet::Some(vec![4, 5])
    );
    assert_eq!(
        ks.intersect(&KeySet::Some(vec![4, 5])),
        KeySet::Some(vec![4, 5])
    );

    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 2])),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![1, 3, 4, 5])),
        KeySet::AllExceptSome(vec![1, 2, 3, 4, 5])
    );
    assert_eq!(
        ks.intersect(&KeySet::AllExceptSome(vec![4, 5])),
        KeySet::AllExceptSome(vec![1, 2, 3, 4, 5])
    );
}
