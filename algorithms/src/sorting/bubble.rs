
pub fn sort<T: PartialOrd>(arr: &mut Vec<T>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_integers() {
        let mut vec1 = vec![3, 1, 2];
        sort(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3]);

        let mut vec2 = vec![4, -1, -3, 2];
        sort(&mut vec2);
        assert_eq!(vec2, vec![-3, -1, 2, 4]);

        let mut vec3 = vec![10];
        sort(&mut vec3);
        assert_eq!(vec3, vec![10]);
    }

    #[test]
    fn sort_characters() {
        let mut vec1 = vec!['c', 'a', 'b'];
        sort(&mut vec1);
        assert_eq!(vec1, vec!['a', 'b', 'c']);

        let mut vec2 = vec!['z', 'x', 'y', 'w'];
        sort(&mut vec2);
        assert_eq!(vec2, vec!['w', 'x', 'y', 'z']);

        let mut vec3 = vec!['m'];
        sort(&mut vec3);
        assert_eq!(vec3, vec!['m']);
    }

    #[test]
    fn sort_binary() {
        let mut vec1 = vec![0b010, 0b001, 0b011];
        sort(&mut vec1);
        assert_eq!(vec1, vec![0b001, 0b010, 0b011]);

        let mut vec2 = vec![0xFF, 0x00, 0xAA, 0x55];
        sort(&mut vec2);
        assert_eq!(vec2, vec![0x00, 0x55, 0xAA, 0xFF]);

        let mut vec3 = vec![0b10101010];
        sort(&mut vec3);
        assert_eq!(vec3, vec![0b10101010]);
    }
}
