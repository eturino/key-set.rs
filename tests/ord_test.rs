use key_set::KeySet;

#[test]
fn test_order() {
    let ks_all = KeySet::All as KeySet<i32>;
    let ks_non = KeySet::None as KeySet<i32>;
    let ks_sm1 = KeySet::Some(vec![0, 4]);
    let ks_sm2 = KeySet::Some(vec![0, 4, 5, 6]);
    let ks_sm3 = KeySet::Some(vec![1, 2, 3]);
    let ks_as1 = KeySet::AllExceptSome(vec![0, 4]);
    let ks_as2 = KeySet::AllExceptSome(vec![0, 4, 5, 6]);
    let ks_as3 = KeySet::AllExceptSome(vec![1, 2, 3]);

    let mut list = vec![
        ks_all.clone(),
        ks_non.clone(),
        ks_sm1.clone(),
        ks_sm3.clone(),
        ks_sm1.clone(),
        ks_sm1.clone(),
        ks_sm2.clone(),
        ks_as1.clone(),
        ks_as3.clone(),
        ks_non.clone(),
        ks_sm1.clone(),
        ks_non.clone(),
        ks_non.clone(),
        ks_all.clone(),
        ks_as2.clone(),
        ks_non.clone(),
        ks_non.clone(),
    ];

    list.sort();
    list.dedup();

    let expected = vec![
        ks_non.clone(),
        ks_sm1.clone(),
        ks_sm2.clone(),
        ks_sm3.clone(),
        ks_as1.clone(),
        ks_as2.clone(),
        ks_as3.clone(),
        ks_all.clone(),
    ];

    assert_eq!(list, expected);
}
