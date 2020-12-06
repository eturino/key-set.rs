use crate::KeySet;
use std::fmt::Debug;

pub fn clean_vec<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();
    list
}

pub fn make_some_with<T>(vec: Vec<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    if vec.len() == 0 {
        KeySet::None
    } else {
        KeySet::Some(vec)
    }
}

pub fn make_all_except_some_with<T>(vec: Vec<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    if vec.len() == 0 {
        KeySet::All
    } else {
        KeySet::AllExceptSome(vec)
    }
}
