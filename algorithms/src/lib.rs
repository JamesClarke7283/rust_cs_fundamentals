#![feature(test)]
pub mod sorting;
pub mod randomize;
use num_traits::{NumCast, PrimInt};

pub trait Sorting {
    fn sort<T: PartialOrd + Clone>(&self, arr: &mut [T]);
}

pub trait Randomize<T>
where
    T: PrimInt + NumCast, // Ensure T supports necessary numeric operations
{
    fn new(seed: T) -> Self;
    fn r#gen(&mut self) -> T;
}


