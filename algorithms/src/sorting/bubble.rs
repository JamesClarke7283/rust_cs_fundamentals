use crate::Sorting;
use crate::sorting::Bubble;

/// Sorts a slice in-place using bubble sort.
///
/// Bubble sort is a simple sorting algorithm that repeatedly steps through the list,
/// compares adjacent elements, and swaps them if they are in the wrong order.
/// The pass through the list is repeated until the list is sorted.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Bubble;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Bubble;
/// let mut numbers = [5, 3, 4, 1, 2];
/// sorter.sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
///
/// Works with different data types:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Bubble;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Bubble;
/// let mut chars = ['c', 'a', 'b', 'e', 'd'];
/// sorter.sort(&mut chars);
/// assert_eq!(chars, ['a', 'b', 'c', 'd', 'e']);
/// ```
///
/// Already sorted data:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Bubble;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Bubble;
/// let mut sorted = [1, 2, 3, 4, 5];
/// sorter.sort(&mut sorted);
/// assert_eq!(sorted, [1, 2, 3, 4, 5]);
/// ```
impl Sorting for Bubble {
    fn sort<T: PartialOrd>(&self, arr: &mut [T]) {
        let mut n = arr.len();
        let mut swapped = true;
        
        while swapped {
            swapped = false;
            for i in 1..n {
                if arr[i - 1] > arr[i] {
                    arr.swap(i - 1, i);
                    swapped = true;
                }
            }
            n -= 1;
        }
    }
}

impl Default for Bubble {
    fn default() -> Self {
        Self
    }
}