use key_set::KeySet;

#[test]
fn test_contains() {
    let ks_none: KeySet<i32> = KeySet::None;
    let ks_all: KeySet<i32> = KeySet::All;
    let ks_some: KeySet<i32> = KeySet::Some(vec![1, 2, 3]);
    let ks_aes: KeySet<i32> = KeySet::AllExceptSome(vec![1, 2, 3]);

    assert!(ks_all.contains(&1));
    assert!(!ks_none.contains(&1));
    assert!(ks_some.contains(&1));
    assert!(!ks_some.contains(&9));
    assert!(!ks_aes.contains(&1));
    assert!(ks_aes.contains(&9));
}
