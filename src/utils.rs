pub fn clean_vec<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();
    list
}
