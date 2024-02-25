use num_traits::{NumCast, PrimInt};
use crate::Randomize;
pub struct LCG<T>
where
    T: PrimInt + NumCast, // Same trait bounds as in the trait definition
{
    a: T,
    c: T,
    m: T,
    state: T,
}

impl<T> LCG<T>
where
    T: PrimInt + NumCast,
{
    const A: u64 = 1_664_525;
    const C: u64 = 1_013_904_223;
    const M: u64 = 2u64.pow(32);
}

impl<T> Randomize<T> for LCG<T>
where
    T: PrimInt + NumCast + std::fmt::Debug,
{
    fn new(seed: T) -> Self {
        let a = T::from(Self::A).expect("Failed to convert A");
        let c = T::from(Self::C).expect("Failed to convert C");
        let m = T::from(Self::M).expect("Failed to convert M");
        println!("a: {a:?}, c: {c:?}, m: {m:?}, seed: {seed:?}"); // Debug print
        Self { a, c, m, state: seed }
    }
    
    

    fn r#gen(&mut self) -> T {
        // Perform the arithmetic using generic operations
        println!("a: {:?}, c: {:?}, m: {:?}, seed: {:?}", self.a, self.c, self.m, self.state);
        self.state = (self.a * self.state + self.c) % self.m;
        self.state
    }
}

impl<T> Default for LCG<T>
where
    T: PrimInt + NumCast + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new(T::from(10).unwrap())
    }
}

