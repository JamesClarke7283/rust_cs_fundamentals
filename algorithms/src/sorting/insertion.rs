use crate::sorting::Insertion;
use crate::Sorting;

/// Sorts a slice in-place using insertion sort.
///
/// Insertion sort is a simple yet efficient algorithm for small to medium-sized lists. 
/// It builds the final sorted list one item at a time by comparing each new item with the 
/// already sorted portion and inserting it into the correct position.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Insertion;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Insertion;
/// let mut numbers = [4, 2, 3, 1, 5];
/// sorter.sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
///
/// Works with different data types:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Insertion;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Insertion;
/// let mut chars = ['e', 'c', 'b', 'd', 'a'];
/// sorter.sort(&mut chars);
/// assert_eq!(chars, ['a', 'b', 'c', 'd', 'e']);
/// ```
///
/// Already sorted data:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Insertion;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Insertion;
/// let mut sorted = [1, 2, 3, 4, 5];
/// sorter.sort(&mut sorted);
/// assert_eq!(sorted, [1, 2, 3, 4, 5]);
/// ```
impl Sorting for Insertion {
    fn sort<T: PartialOrd>(&self, arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

impl Default for Insertion {
    fn default() -> Self {
        Insertion
    }
}