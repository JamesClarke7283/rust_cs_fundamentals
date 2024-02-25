use crate::Searching;
use std::cmp::Ordering;

/// Performs a binary search on a sorted iterable collection.
///
/// This search method halves the collection size with each iteration by comparing the middle element with the target value.
/// If the target value matches the middle element, its index is returned.
/// If the target value is less than the middle element, the search continues on the left half of the collection.
/// Otherwise, the search continues on the right half of the collection.
/// The function returns `Some(index)` of the found element, or `None` if not found.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use cs_fundamentals_algorithms::searching::Binary;
/// use cs_fundamentals_algorithms::Searching;
///
/// let searcher = Binary;
/// let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// assert_eq!(Some(2), Binary::search(&numbers, &3));
/// assert_eq!(None, Binary::search(&numbers, &10));
/// ```
///
/// Note: The collection must be sorted for binary search to work correctly.
///
/// ```
/// use cs_fundamentals_algorithms::searching::Binary;
/// use cs_fundamentals_algorithms::Searching;
///
/// let searcher = Binary;
/// let mut numbers = vec![9, 1, 4, 3, 7, 8, 2, 5, 6];
/// numbers.sort();
/// assert_eq!(Some(4), Binary::search(&numbers, &5));
/// ```
impl Searching for super::Binary {
    fn search<I, K>(iterable: I, key: K) -> Option<usize>
    where
        I: IntoIterator<Item = K>,
        K: Ord,
    {
        let mut vec: Vec<K> = iterable.into_iter().collect();
        vec.sort_unstable(); // Ensure the vector is sorted for binary search
        let mut low = 0;
        let mut high = vec.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let mid_value = &vec[mid];

            match mid_value.cmp(&key) {
                Ordering::Equal => return Some(mid),
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid - 1,
            }            
        }
        None
    }
}