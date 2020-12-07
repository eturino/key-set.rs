use key_set::KeySet;

#[test]
fn test_all_serialize() {
    let expected = "\"All\"".to_string();
    let ks: KeySet<i32> = KeySet::All;
    let j = serde_json::to_string(&ks).unwrap();
    assert_eq!(j, expected);
}

#[test]
fn test_all_deserialize() {
    let json = "\"All\"";
    let ks: KeySet<i32> = serde_json::from_str(json).unwrap();
    assert_eq!(ks, KeySet::All);
}

#[test]
fn test_none_serialize() {
    let expected = "\"None\"".to_string();
    let ks: KeySet<i32> = KeySet::None;
    let j = serde_json::to_string(&ks).unwrap();
    assert_eq!(j, expected);
}

#[test]
fn test_none_deserialize() {
    let json = "\"None\"";
    let ks: KeySet<i32> = serde_json::from_str(json).unwrap();
    assert_eq!(ks, KeySet::None);
}

#[test]
fn test_some_serialize() {
    let expected = "{\"Some\":[1,2,3]}".to_string();
    let ks: KeySet<i32> = KeySet::for_some(&vec![1, 2, 3]);
    let j = serde_json::to_string(&ks).unwrap();
    assert_eq!(j, expected);
}

#[test]
fn test_some_deserialize() {
    let json = "{\"Some\":[1,2,3]}";
    let ks: KeySet<i32> = serde_json::from_str(json).unwrap();
    assert_eq!(ks, KeySet::Some(vec![1, 2, 3]));
}

#[test]
fn test_all_except_some_serialize() {
    let expected = "{\"AllExceptSome\":[1,2,3]}".to_string();
    let ks: KeySet<i32> = KeySet::for_all_except_some(&vec![1, 2, 3]);
    let j = serde_json::to_string(&ks).unwrap();
    assert_eq!(j, expected);
}

#[test]
fn test_all_except_some_deserialize() {
    let json = "{\"AllExceptSome\":[1,2,3]}";
    let ks: KeySet<i32> = serde_json::from_str(json).unwrap();
    assert_eq!(ks, KeySet::AllExceptSome(vec![1, 2, 3]));
}
