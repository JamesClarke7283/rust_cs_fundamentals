pub mod sorting;

pub trait Sorting {
    fn sort<T: PartialOrd + Clone>(&self, arr: &mut [T]);
}