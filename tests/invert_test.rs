use key_set::KeySet;

#[test]
fn test_invert() {
    let ks_none: KeySet<i32> = KeySet::None;
    let ks_all: KeySet<i32> = KeySet::All;
    let ks_some: KeySet<i32> = KeySet::Some(vec![1, 2, 3]);
    let ks_aes: KeySet<i32> = KeySet::AllExceptSome(vec![1, 2, 3]);

    assert_eq!(ks_all.invert(), KeySet::None);
    assert_eq!(ks_none.invert(), KeySet::All);
    assert_eq!(ks_aes.invert(), KeySet::Some(vec![1, 2, 3]));
    assert_eq!(ks_some.invert(), KeySet::AllExceptSome(vec![1, 2, 3]));
}
