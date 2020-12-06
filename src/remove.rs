use crate::utils::{clean_vec, make_all_except_some_with, make_some_with};
use crate::KeySet;
use std::fmt::Debug;

pub fn remove_from_all<T>(other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    match other {
        KeySet::All => KeySet::None,
        KeySet::None => KeySet::All,
        KeySet::Some(e) => KeySet::AllExceptSome(e.clone()),
        KeySet::AllExceptSome(e) => KeySet::Some(e.clone()),
    }
}

pub fn remove_from_none<T>(_other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    KeySet::None
}

pub fn remove_from_some<T>(elements: &Vec<T>, other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    match other {
        KeySet::All => KeySet::None,
        KeySet::None => KeySet::Some(elements.clone()),
        KeySet::AllExceptSome(e) => {
            let mut keys_in_both = elements.clone();
            keys_in_both.retain(|x| e.contains(x));
            make_some_with(clean_vec(keys_in_both))
        }
        KeySet::Some(e) => {
            let mut mine_except_others = elements.clone();
            mine_except_others.retain(|x| !e.contains(x));
            make_some_with(clean_vec(mine_except_others))
        }
    }
}

pub fn remove_from_all_except_some<T>(elements: &Vec<T>, other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    match other {
        KeySet::All => KeySet::None,
        KeySet::None => KeySet::AllExceptSome(elements.clone()),
        KeySet::AllExceptSome(e) => {
            let mut other_except_mine = e.clone();
            other_except_mine.retain(|x| !elements.contains(x));
            make_some_with(clean_vec(other_except_mine))
        }
        KeySet::Some(e) => {
            let mut both_vectors = elements.clone();
            both_vectors.extend(e);
            make_all_except_some_with(clean_vec(both_vectors))
        }
    }
}
