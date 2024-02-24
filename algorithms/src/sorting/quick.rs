use crate::sorting::Quick;
use crate::Sorting;

/// Sorts a slice in-place using quicksort.
///
/// Quicksort is an efficient, divide-and-conquer, comparison-based sorting algorithm. It picks an element as a pivot and partitions the given array around the picked pivot. There are different versions of quicksort that pick pivot in different ways:
///
/// - Always pick the first element as the pivot.
/// - Always pick the last element as the pivot (implemented below).
/// - Pick a random element as the pivot.
/// - Pick the median as the pivot.
///
/// The key process in quicksort is partitioning. The target of partitions is, given an array and an element x of an array as the pivot, put x at its correct position in the sorted array and put all smaller elements (smaller than x) before x, and put all greater elements (greater than x) after x. All this should be done in linear time.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Quick;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Quick;
/// let mut numbers = [9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
/// sorter.sort(&mut numbers);
/// assert_eq!(numbers, [2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);
/// ```
///
/// Works with different data types:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Quick;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Quick;
/// let mut chars = ['e', 'a', 'c', 'b', 'd'];
/// sorter.sort(&mut chars);
/// assert_eq!(chars, ['a', 'b', 'c', 'd', 'e']);
/// ```
///
/// Already sorted data:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Quick;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Quick;
/// let mut sorted = [1, 2, 3, 4, 5];
/// sorter.sort(&mut sorted);
/// assert_eq!(sorted, [1, 2, 3, 4, 5]);
/// ```
///
/// Sorting with duplicates:
///
/// ```
/// use cs_fundamentals_algorithms::sorting::Quick;
/// use cs_fundamentals_algorithms::Sorting;
///
/// let sorter = Quick;
/// let mut numbers = [5, 3, 5, 3, 2];
/// sorter.sort(&mut numbers);
/// assert_eq!(numbers, [2, 3, 3, 5, 5]);
/// ```
impl Sorting for Quick {
    fn sort<T: PartialOrd + Clone>(&self, arr: &mut [T]) {
        quicksort(arr, 0, arr.len() as isize - 1);
    }
}

fn quicksort<T: PartialOrd + Clone>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        quicksort(arr, low, pivot_index - 1);
        quicksort(arr, pivot_index + 1, high);
    }
}

fn partition<T: PartialOrd + Clone>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize].clone();
    let mut i = low - 1;
    for j in low..high {
        if arr[j as usize] < pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize);
    i + 1
}

impl Default for Quick {
    fn default() -> Self {
        Self
    }
}