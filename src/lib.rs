use crate::utils::clean_vec;
use std::fmt::Debug;

mod utils;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum KeySet<T>
where
    T: Ord + Debug,
{
    All,
    None,
    Some(Vec<T>),
    AllExceptSome(Vec<T>),
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
        T: Ord + Debug,
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
