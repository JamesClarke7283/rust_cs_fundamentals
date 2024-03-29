
pub mod utils;
pub mod bubble;
pub mod insertion;
pub mod merge;
pub mod quick;

pub struct Bubble;

pub struct Insertion;

pub struct Merge;

pub struct Quick;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sorting;

    fn run_sort_test<S: Sorting + Default>(sorter: S) {
        let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
        sorter.sort(&mut numbers);
        assert_eq!(numbers, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    
        let mut characters = vec!['d', 'a', 'b', 'c'];
        sorter.sort(&mut characters);
        assert_eq!(characters, vec!['a', 'b', 'c', 'd']);
    
        let mut binary_elements: Vec<u8> = vec![0b00001111, 0b11110000, 0b10101010, 0b01010101];
        sorter.sort(&mut binary_elements);
        assert_eq!(binary_elements, vec![0b00001111, 0b01010101, 0b10101010, 0b11110000]);
    }
    

    #[test]
    fn test_bubble_sort() {
        run_sort_test(Bubble::default());
    }

    #[test]
    fn test_insertion_sort() {
        run_sort_test(Insertion::default());
    }

    #[test]
    fn test_merge_sort() {
        run_sort_test(Merge::default());
    }

    #[test]
    fn test_quick_sort() {
        run_sort_test(Quick::default());
    }

}


#[cfg(test)]
mod benches {
    extern crate test;
    use super::*;
    use crate::Sorting;
    use crate::randomize::lcg::LCG;
    use test::Bencher;
    use crate::Randomize; 

    fn run_sort_bench<S: Sorting + Default>(b: &mut Bencher, sorter: S) {
        // Initialize your LCG. Assuming the seed is of type u32 for simplicity.
        let mut lcg_rng = LCG::<u64>::new(12345); // Use a fixed seed for reproducibility

        // Generate numbers using LCG
        let mut numbers: Vec<u64> = (0..10_000).map(|_| lcg_rng.r#gen()).collect();

        b.iter(|| {
            sorter.sort(&mut numbers);
        });
    }

    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        run_sort_bench(b, Bubble::default());
    }

    #[bench]
    fn bench_insertion_sort(b: &mut Bencher) {
        run_sort_bench(b, Insertion::default());
    }

    #[bench]
    fn bench_merge_sort(b: &mut Bencher) {
        run_sort_bench(b, Merge::default());
    }

    #[bench]
    fn bench_quick_sort(b: &mut Bencher) {
        run_sort_bench(b, Quick::default());
    }
}







