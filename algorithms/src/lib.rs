#![feature(test)]
pub mod sorting;
pub mod randomize;
pub mod searching;
use num_traits::{NumCast, PrimInt};

pub trait Sorting {
    fn sort<T: PartialOrd + Clone>(&self, arr: &mut [T]);
}

pub trait Randomize<T>
where
    T: PrimInt + NumCast,
{
    fn new(seed: T) -> Self;
    fn r#gen(&mut self) -> T;
}

pub trait Searching {
    // Defines a generic search function within the trait.
    // `iterable` is a generic parameter representing any iterable collection,
    // and `key` is the item you're searching for within that collection.
    // The function returns an Option<usize>, representing the index of the found item, or None if not found.
    fn search<I, K>(iterable: I, key: K) -> Option<usize>
    where
        I: IntoIterator<Item = K>,
        K: Ord;
}





