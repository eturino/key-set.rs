use key_set::KeySet;

#[test]
fn test_some_with_elements() {
    let v = vec![1, 2, 1, 4, 0];
    let actual = KeySet::some(&v);
    assert_eq!(actual, KeySet::Some(vec![0, 1, 2, 4]));
    assert_eq!(v, vec![1, 2, 1, 4, 0]);
}

#[test]
fn test_some_with_blank() {
    let actual = KeySet::some(&vec![] as &Vec<i32>);
    assert_eq!(actual, KeySet::None);
}

#[test]
fn test_all_except_some_with_elements() {
    let v = vec![1, 2, 1, 4, 0];
    let actual = KeySet::all_except_some(&v);
    assert_eq!(actual, KeySet::AllExceptSome(vec![0, 1, 2, 4]));
    assert_eq!(v, vec![1, 2, 1, 4, 0]);
}

#[test]
fn test_all_except_some_with_blank() {
    let actual = KeySet::all_except_some(&vec![] as &Vec<i32>);
    assert_eq!(actual, KeySet::All);
}

#[test]
fn test_clone() {
    let ks_none: KeySet<i32> = KeySet::None;
    let ks_all: KeySet<i32> = KeySet::All;
    let ks_some: KeySet<i32> = KeySet::Some(vec![1, 2, 3]);
    let ks_aes: KeySet<i32> = KeySet::AllExceptSome(vec![1, 2, 3]);

    assert_eq!(ks_none.clone(), KeySet::None);
    assert_eq!(ks_all.clone(), KeySet::All);
    assert_eq!(ks_some.clone(), KeySet::Some(vec![1, 2, 3]));
    assert_eq!(ks_aes.clone(), KeySet::AllExceptSome(vec![1, 2, 3]));
}

#[test]
fn test_factory_some() {
    let v_empty: Vec<i32> = vec![];
    let vec1 = vec![3, 1, 2, 1];

    assert_eq!(vec1, vec![3, 1, 2, 1]);
    assert_eq!(KeySet::some(&vec1), KeySet::Some(vec![1, 2, 3]));
    assert_eq!(vec1, vec![3, 1, 2, 1]); // does not mutate the given list

    assert_eq!(KeySet::some(&v_empty), KeySet::None);
}

#[test]
fn test_factory_all_except_some() {
    let v_empty: Vec<i32> = vec![];
    let vec1 = vec![3, 1, 2, 1];

    assert_eq!(vec1, vec![3, 1, 2, 1]);
    assert_eq!(
        KeySet::all_except_some(&vec1),
        KeySet::AllExceptSome(vec![1, 2, 3])
    );
    assert_eq!(vec1, vec![3, 1, 2, 1]); // does not mutate the given list

    assert_eq!(KeySet::all_except_some(&v_empty), KeySet::All);
}
