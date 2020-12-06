use crate::utils::clean_vec;
use std::fmt::Debug;

mod utils;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum KeySet<T>
where
    T: Ord + Debug + Clone,
{
    All,
    None,
    Some(Vec<T>),
    AllExceptSome(Vec<T>),
}

impl<T> KeySet<T>
where
    T: Ord + Debug + Clone,
{
    fn invert(&self) -> Self {
        match self {
            KeySet::All => KeySet::None,
            KeySet::None => KeySet::All,
            KeySet::Some(e) => key_set_all_except_some(e),
            KeySet::AllExceptSome(e) => key_set_some(e),
        }
    }

    fn contains(&self, element: &T) -> bool {
        match self {
            KeySet::All => true,
            KeySet::None => false,
            KeySet::Some(e) => e.contains(&element),
            KeySet::AllExceptSome(e) => !e.contains(&element),
        }
    }
}

impl<T> std::fmt::Display for KeySet<T>
where
    T: Ord + Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KeySet::All => write!(f, "KeySet::All"),
            KeySet::None => write!(f, "KeySet::None"),
            KeySet::Some(e) => write!(f, "KeySet::Some({:?})", e),
            KeySet::AllExceptSome(e) => write!(f, "KeySet::AllExceptSome({:?})", e),
        }
    }
}

impl<T> Clone for KeySet<T>
where
    T: Ord + Debug + Clone,
{
    fn clone(&self) -> Self {
        match self {
            KeySet::All => KeySet::All,
            KeySet::None => KeySet::None,
            KeySet::Some(e) => key_set_some(e),
            KeySet::AllExceptSome(e) => key_set_all_except_some(e),
        }
    }
}

pub fn key_set_some<T>(elements: &Vec<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone,
{
    let vec = clean_vec(elements.clone());
    if vec.len() == 0 {
        KeySet::None
    } else {
        KeySet::Some(vec)
    }
}

pub fn key_set_all_except_some<T>(elements: &Vec<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone,
{
    let vec = clean_vec(elements.clone());
    if vec.len() == 0 {
        KeySet::All
    } else {
        KeySet::AllExceptSome(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_type<T>(_ks: KeySet<T>)
    where
        T: Ord + Debug + Clone,
    {
        assert!(true); // this is just to check compiler
    }

    #[test]
    fn test_creation() {
        let ks_none: KeySet<i32> = KeySet::None;
        let ks_all: KeySet<i32> = KeySet::All;
        check_type(ks_none);
        check_type(ks_all);
        check_type(KeySet::Some(vec![1, 2, 3]));
        check_type(KeySet::Some(vec!['a', 'b', 'c']));
        check_type(KeySet::Some(vec!["a", "b", "c"]));
        check_type(KeySet::AllExceptSome(vec![1, 2, 3]));
        check_type(KeySet::AllExceptSome(vec!['a', 'b', 'c']));
        check_type(KeySet::AllExceptSome(vec!["a", "b", "c"]));
    }

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

    #[test]
    fn test_key_set_some() {
        let v_empty: Vec<i32> = vec![];
        let vec1 = vec![3, 1, 2, 1];

        assert_eq!(vec1, vec![3, 1, 2, 1]);
        assert_eq!(key_set_some(&vec1), KeySet::Some(vec![1, 2, 3]));
        assert_eq!(vec1, vec![3, 1, 2, 1]); // does not mutate the given list

        assert_eq!(key_set_some(&v_empty), KeySet::None);
    }

    #[test]
    fn test_key_set_all_except_some() {
        let v_empty: Vec<i32> = vec![];
        let vec1 = vec![3, 1, 2, 1];

        assert_eq!(vec1, vec![3, 1, 2, 1]);
        assert_eq!(
            key_set_all_except_some(&vec1),
            KeySet::AllExceptSome(vec![1, 2, 3])
        );
        assert_eq!(vec1, vec![3, 1, 2, 1]); // does not mutate the given list

        assert_eq!(key_set_all_except_some(&v_empty), KeySet::All);
    }
}
