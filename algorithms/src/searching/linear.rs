use crate::Searching;

/// Performs a linear search on an iterable collection.
///
/// This search method iterates through each item in the collection, comparing it with the
/// search key. It returns the index of the first occurrence of the key if found, or `None` otherwise.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use cs_fundamentals_algorithms::searching::Linear;
/// use cs_fundamentals_algorithms::Searching;
///
/// let searcher = Linear;
/// let numbers = vec![2, 3, 5, 7, 11];
/// assert_eq!(Some(3), Linear::search(&numbers, &7));
/// assert_eq!(None, Linear::search(&numbers, &4));
///
/// let letters = vec!['a', 'b', 'c', 'd'];
/// assert_eq!(Some(2), Linear::search(&letters, &'c'));
/// assert_eq!(None, Linear::search(&letters, &'e'));
/// ```
impl Searching for super::Linear {
    fn search<I, K>(iterable: I, key: K) -> Option<usize>
    where
        I: IntoIterator<Item = K>,
        K: PartialEq,
    {
        iterable.into_iter().enumerate().find_map(|(index, item)| {
            if item == key {
                Some(index)
            } else {
                None
            }
        })
    }
}