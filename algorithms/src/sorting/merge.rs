use crate::sorting::utils::merge;
use crate::sorting::Merge;
use crate::Sorting;

/// Sorts a slice in-place using merge sort.
///
/// Merge sort is an efficient, stable, comparison-based, divide and conquer sorting algorithm.
/// Most implementations produce a stable sort, meaning that the implementation preserves the input order of equal elements in the sorted output.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Merge;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Merge;
/// let mut numbers = [9, 5, 2, 3, 6, 8, 1, 4, 7];
/// sorter.sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
///
/// Works with different data types:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Merge;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Merge;
/// let mut chars = ['e', 'c', 'b', 'd', 'a', 'g', 'f'];
/// sorter.sort(&mut chars);
/// assert_eq!(chars, ['a', 'b', 'c', 'd', 'e', 'f', 'g']);
/// ```
///
/// Sorting a vector of strings:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Merge;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Merge;
/// let mut words = ["banana", "apple", "orange", "mango"];
/// sorter.sort(&mut words);
/// assert_eq!(words, ["apple", "banana", "mango", "orange"]);
/// ```
impl Sorting for Merge {
    fn sort<T: PartialOrd + Clone>(&self, arr: &mut [T]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        // Sort the left and right halves
        Self::sort(&self, &mut left);
        Self::sort(&self, &mut right);

        // Merge the sorted halves
        merge(arr, &left, &right);
    }
}

impl Default for Merge {
    fn default() -> Self {
        Merge
    }
}