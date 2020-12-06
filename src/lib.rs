use crate::intersect::{
    intersect_from_all, intersect_from_all_except_some, intersect_from_none, intersect_from_some,
};
use crate::remove::{
    remove_from_all, remove_from_all_except_some, remove_from_none, remove_from_some,
};
use crate::utils::clean_vec;
use std::fmt::Debug;

mod intersect;
mod remove;
mod utils;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    None,
    Some(Vec<T>),
    AllExceptSome(Vec<T>),
    All,
}

impl<T> KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
{
    pub fn some(elements: &Vec<T>) -> KeySet<T> {
        let vec = clean_vec(elements.clone());
        utils::make_some_with(vec)
    }

    pub fn all_except_some(elements: &Vec<T>) -> KeySet<T> {
        let vec = clean_vec(elements.clone());
        utils::make_all_except_some_with(vec)
    }

    pub fn invert(&self) -> Self {
        match self {
            KeySet::All => KeySet::None,
            KeySet::None => KeySet::All,
            KeySet::Some(e) => KeySet::AllExceptSome(e.clone()),
            KeySet::AllExceptSome(e) => KeySet::Some(e.clone()),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        match self {
            KeySet::All => true,
            KeySet::None => false,
            KeySet::Some(e) => e.contains(&element),
            KeySet::AllExceptSome(e) => !e.contains(&element),
        }
    }

    pub fn remove(&self, other: &KeySet<T>) -> KeySet<T> {
        match self {
            KeySet::All => remove_from_all(other),
            KeySet::None => remove_from_none(other),
            KeySet::Some(e) => remove_from_some(e, other),
            KeySet::AllExceptSome(e) => remove_from_all_except_some(e, other),
        }
    }

    pub fn intersect(&self, other: &KeySet<T>) -> KeySet<T> {
        match self {
            KeySet::All => intersect_from_all(other),
            KeySet::None => intersect_from_none(other),
            KeySet::Some(e) => intersect_from_some(e, other),
            KeySet::AllExceptSome(e) => intersect_from_all_except_some(e, other),
        }
    }
}

impl<T> std::fmt::Display for KeySet<T>
where
    T: Ord + Debug + Clone + Copy,
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
    T: Ord + Debug + Clone + Copy,
{
    fn clone(&self) -> Self {
        match self {
            KeySet::All => KeySet::All,
            KeySet::None => KeySet::None,
            KeySet::Some(e) => KeySet::Some(e.clone()),
            KeySet::AllExceptSome(e) => KeySet::AllExceptSome(e.clone()),
        }
    }
}
