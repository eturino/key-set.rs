use crate::utils::{clean_vec, make_all_except_some_with, make_some_with};
use crate::KeySet;
use std::fmt::Debug;

pub fn intersect_from_all<T>(other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone,
{
    other.clone()
}

pub fn intersect_from_none<T>(_other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone,
{
    KeySet::None
}

pub fn intersect_from_some<T>(elements: &Vec<T>, other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone,
{
    match other {
        KeySet::All => KeySet::Some(elements.clone()),
        KeySet::None => KeySet::None,

        KeySet::Some(e) => {
            let mut keys_in_both = elements.clone();
            keys_in_both.retain(|x| e.contains(x));
            make_some_with(clean_vec(keys_in_both))
        }

        KeySet::AllExceptSome(e) => {
            let mut keys_not_in_other = elements.clone();
            keys_not_in_other.retain(|x| !e.contains(x));
            make_some_with(clean_vec(keys_not_in_other))
        }
    }
}

pub fn intersect_from_all_except_some<T>(elements: &Vec<T>, other: &KeySet<T>) -> KeySet<T>
where
    T: Ord + Debug + Clone,
{
    match other {
        KeySet::All => KeySet::AllExceptSome(elements.clone()),
        KeySet::None => KeySet::None,

        KeySet::Some(e) => {
            let mut others_not_mine = e.clone();
            others_not_mine.retain(|x| !elements.contains(x));
            make_some_with(clean_vec(others_not_mine))
        }

        KeySet::AllExceptSome(e) => {
            let mut both_vectors = elements.clone();
            both_vectors.extend(e.clone());
            make_all_except_some_with(clean_vec(both_vectors))
        }
    }
}
