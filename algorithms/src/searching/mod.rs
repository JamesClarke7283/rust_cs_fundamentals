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