# Rust crate `key_set`

KeySet representing concepts of All, None, Some, and AllExceptSome, the last 2 with a sorted uniq list of keys, and all with intersection and diff calculations.

[Github repo here](https://github.com/eturino/key-set.rs)

Other versions:
- TypeScript: <https://github.com/eturino/ts-key-set>
- Ruby: <https://github.com/eturino/ruby_key_set>

## Usage

We have an enum with:

- `KeySet::All` represents the entirety of possible keys (`𝕌`)
- `KeySet::None` represents an empty set (`∅`)
- `KeySet::Some(vec)` represents a concrete set (`A ⊂ 𝕌`)
- `KeySet::AllExceptSome(vec)` represents the complementary set of a set, all the elements except the given ones (`A' = {x ∈ 𝕌 | x ∉ A}`) _(see [Complement in Wikipedia](https://en.wikipedia.org/wiki/Complement_\(set*theory\)))*

We can have a KeySet of `T` where `T: Ord + Debug + Clone`

KeySet implements `cmp::Ord`, `cmp::PartialOrd`, `cmp::Eq`, `cmp::PartialEq`, `std::fmt::Debug`, and `std::fmt::Display`


### Creation: `KeySet::for_some(&list)`, `KeySet::for_all_except_some(&list)`

Build your KeySets using the factory functions, giving

- To get a KeySet that represents the given list: `KeySet::for_some(&list)`
    - if the list is empty, we'll get `None`
    - otherwise, we'll get `Some`
- To get a KeySet that represents the complementary set of the given list: `KeySet::for_all_except_some(&list)`
    - if the list is empty, we'll get `All`
    - otherwise, we'll get `AllExceptSome`

```rust
fn example() {
    let empty_vector: Vec<i32> = vec![];

    let ks1 = KeySet::for_some(&empty_vector); // => KeySet::None
    let ks2 = KeySet::for_some(&vec![1, 2, 3]); // => KeySet::Some([1, 2, 3])
    let ks3 = KeySet::for_all_except_some(empty_vector); // => KeySet::All
    let ks4 = KeySet::for_all_except_some(&vec![1, 2, 3]); // => KeySet::AllExceptSome([1, 2, 3])
}
```

### `contains(&element)`

Returns a boolean defining if the KeySet includes the given element.

```rust
fn example() {
    let ks1 = KeySet::for_some(vec![1, 2, 3]); // => KeySet::Some([1, 2, 3])
    ks1.contains(&1); // => true
    ks1.contains(&7); // => false

    let ks2: KeySet<i32> = KeySet::All;
    ks2.contains(&1); // => true
    ks2.contains(&7); // => true

    let ks3: KeySet<i32> = KeySet::None;
    ks3.contains(&1); // => false
    ks3.contains(&7); // => false
    
    let ks4 = KeySet::for_all_except_some(vec![1, 2, 3]); // => KeySet::AllExceptSome([1, 2, 3])
    ks4.contains(&1); // => false
    ks4.contains(&7); // => true
}
```

### `invert()`

All KeySet has an `invert()` method that returns an instance of the opposite class, which represents the complementary KeySet. _(see [Complement in Wikipedia](https://en.wikipedia.org/wiki/Complement_\(set*theory\)))*

- `All` ⟷ `None`
- `Some` ⟷ `AllExceptSome`

```rust
fn example() {
    let key_set = KeySet::for_some(vec![1, 2, 3]); // => KeySet::Some([1, 2, 3])
    let comp = key_set.clone(); // => KeySet::AllExceptSome([1, 2, 3])
}
```

### `remove(&other)`

Returns a new KeySet with the difference between ThisSet - OtherSet `(A - B)`

```rust
fn example() {
    let key_set = KeySet::for_some(vec![1, 2, 3]); // => KeySet::Some([1, 2, 3])
    let other = KeySet::for_some(vec![1, 3, 4]); // => KeySet::Some([1, 2, 3])
    let comp = key_set.remove(&other); // => KeySet::Some([2])
}
```

### `intersect(&other)`

Returns a new KeySet with the intersection of both Sets `(A ∩ B)`, representing the elements present in both sets


```rust
fn example() {
    let key_set = KeySet::for_some(vec![1, 2, 3]); // => KeySet::Some([1, 2, 3])
    let other = KeySet::for_some(vec![1, 3, 4]); // => KeySet::Some([1, 2, 3])
    let comp = key_set.intersect(&other); // => KeySet::Some([1, 3])
}
```


### `clone()`

All KeySet has a `clone()` method, which will return a new instance of the same class that represents the same KeySet.

If the KeySet is `KeySetSome` or `KeySetAllExceptSome`, they will have a vector with the same keys.

```rust
fn example() {
    let key_set = KeySet::for_some(vec![1, 2, 3]); // => KeySet::Some([1, 2, 3])
    let comp = key_set.clone(); // => KeySet::Some([1, 2, 3])
    let equal = key_set == comp; // => true
}
```
