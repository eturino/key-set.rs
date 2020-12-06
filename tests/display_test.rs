use key_set::KeySet;

#[test]
fn test_display() {
    let ks_none: KeySet<i32> = KeySet::None;
    let ks_all: KeySet<i32> = KeySet::All;
    let ks_some: KeySet<i32> = KeySet::Some(vec![1, 2, 3]);
    let ks_aes: KeySet<i32> = KeySet::AllExceptSome(vec![1, 2, 3]);

    assert_eq!(format!("Display: {}", ks_none), "Display: KeySet::None");
    assert_eq!(format!("Display: {}", ks_all), "Display: KeySet::All");
    assert_eq!(
        format!("Display: {}", ks_some),
        "Display: KeySet::Some([1, 2, 3])"
    );
    assert_eq!(
        format!("Display: {}", ks_aes),
        "Display: KeySet::AllExceptSome([1, 2, 3])"
    );
}
