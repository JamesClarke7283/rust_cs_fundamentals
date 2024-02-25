pub mod binary;
pub mod linear;
pub struct Linear;
pub struct Binary;

#[cfg(test)]
mod tests {
    use crate::Searching;

    fn run_search_test<S: Searching>(_searcher: S) {
        let numbers = vec![1, 2 ,3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Some(2), S::search(numbers.clone(), 3));
        assert_eq!(Some(4), S::search(numbers.clone(), 5));
        assert_eq!(None, S::search(numbers, 10));

        let characters = vec!['a', 'b', 'c', 'd'];
        assert_eq!(Some(0), S::search(characters.clone(), 'a'));
        assert_eq!(None, S::search(characters, 'z'));
    }

    #[test]
    fn test_linear_search() {
        run_search_test(super::Linear);
    }

    #[test]
    fn test_binary_search() {
         run_search_test(super::Binary);
    }
}

extern crate test;

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;
    use crate::{Searching, randomize::lcg::LCG};
    use crate::Randomize;

    // Function to generate a sorted Vec<u64> using LCG
    fn generate_sorted_vec(size: usize, seed: u64) -> Vec<u64> {
        let mut lcg = LCG::new(seed);
        let mut numbers: Vec<u64> = (0..size).map(|_| lcg.r#gen()).collect();
        numbers.sort();
        numbers
    }

    // Function to generate an unsorted Vec<u64> using LCG
    fn generate_unsorted_vec(size: usize, seed: u64) -> Vec<u64> {
        let mut lcg = LCG::new(seed);
        (0..size).map(|_| lcg.r#gen()).collect()
    }

    // Benchmarks for Linear Search
    #[bench]
    fn bench_linear_search_sorted(b: &mut Bencher) {
        let numbers = generate_sorted_vec(10_000, 42);
        b.iter(|| {
            // Example key we know exists for demonstration purposes
            Linear::search(&numbers, &numbers[5000]);
        });
    }

    #[bench]
    fn bench_linear_search_unsorted(b: &mut Bencher) {
        let numbers = generate_unsorted_vec(10_000, 42);
        b.iter(|| {
            // Example key we know exists for demonstration purposes
            Linear::search(&numbers, &numbers[5000]);
        });
    }

    // Benchmarks for Binary Search
    #[bench]
    fn bench_binary_search_sorted(b: &mut Bencher) {
        let numbers = generate_sorted_vec(10_000, 42);
        b.iter(|| {
            // Example key we know exists for demonstration purposes
            Binary::search(&numbers, &numbers[5000]);
        });
    }
}