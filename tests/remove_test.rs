use key_set::KeySet;

#[test]
fn test_all() {
    let key_set: KeySet<i32> = KeySet::All;

    assert_eq!(key_set.remove(&KeySet::All), KeySet::None as KeySet<i32>);
    assert_eq!(key_set.remove(&KeySet::None), KeySet::All as KeySet<i32>);
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![1, 2, 3])),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::Some(vec![1, 2, 3])
    );
}

#[test]
fn test_some() {
    let key_set = KeySet::Some(vec![1, 2, 3]);

    assert_eq!(key_set.remove(&KeySet::All), KeySet::None as KeySet<i32>);
    assert_eq!(key_set.remove(&KeySet::None), KeySet::Some(vec![1, 2, 3]));
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![1, 2])),
        KeySet::Some(vec![3])
    );
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![1, 2, 3])),
        KeySet::None as KeySet<i32>
    );
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![4, 5])),
        KeySet::Some(vec![1, 2, 3])
    );

    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![1, 2])),
        KeySet::Some(vec![1, 2])
    );
    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::Some(vec![1, 2, 3])
    );
    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![4, 5])),
        KeySet::None as KeySet<i32>
    );
}

#[test]
fn test_all_except_some() {
    let key_set = KeySet::AllExceptSome(vec![1, 2, 3]);

    assert_eq!(key_set.remove(&KeySet::All), KeySet::None as KeySet<i32>);
    assert_eq!(
        key_set.remove(&KeySet::None),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![1, 2])),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![1, 2, 3])),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![4, 5])),
        KeySet::AllExceptSome(vec![1, 2, 3, 4, 5])
    );

    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![1, 2])),
        KeySet::None as KeySet<i32>
    );
    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::None as KeySet<i32>
    );
    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![4, 5])),
        KeySet::Some(vec![4, 5])
    );
    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![1, 4, 5])),
        KeySet::Some(vec![4, 5])
    );
}

#[test]
fn test_none() {
    let key_set: KeySet<i32> = KeySet::None;
    assert_eq!(key_set.remove(&KeySet::All), KeySet::None as KeySet<i32>);
    assert_eq!(key_set.remove(&KeySet::None), KeySet::None as KeySet<i32>);
    assert_eq!(
        key_set.remove(&KeySet::Some(vec![1, 2, 3])),
        KeySet::None as KeySet<i32>
    );
    assert_eq!(
        key_set.remove(&KeySet::AllExceptSome(vec![1, 2, 3])),
        KeySet::None as KeySet<i32>
    );
}
