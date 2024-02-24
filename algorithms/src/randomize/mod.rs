pub mod lcg;
#[allow(unused_imports)]
use crate::Randomize;

#[cfg(test)]
mod randomness_tests {
    use super::*; // Import necessary items from the outer module
    use crate::randomize::lcg::LCG; // Ensure this path matches the location of your LCG implementation
    use std::collections::HashSet;

    // Function to test a specific RNG implementation for patterns and repeats
    fn test_rng_pattern_and_repeats<R: Randomize<u64> + Default>() {
        let mut rng = R::default(); // Assuming your RNG implements Default for initialization
        let mut numbers = Vec::new();
        let mut seen = HashSet::new();
        let mut is_doubling = true;
        let mut last_number: Option<u64> = None;
        let mut has_repeats = false;

        // Generate a sequence of random numbers
        for _ in 0..100 {
            let number = rng.r#gen();
            numbers.push(number);

            // Check for doubling pattern
            if let Some(last) = last_number {
                if number != last * 2 {
                    is_doubling = false;
                }
            }
            last_number = Some(number);

            // Check for repeats
            if !seen.insert(number) {
                has_repeats = true;
            }
        }

        // Assert conditions based on your requirements
        assert!(!is_doubling, "The sequence should not be just doubling every time.");
        assert!(!has_repeats, "The sequence should not have repeated numbers.");
    }

    #[test]
    fn test_lcg_rng() {
        test_rng_pattern_and_repeats::<LCG<u64>>();
    }
}

