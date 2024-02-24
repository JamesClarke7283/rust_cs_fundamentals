pub fn merge<T: PartialOrd + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut merged_idx = 0;

    // Merge the left and right arrays
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            arr[merged_idx] = left[left_idx].clone();
            left_idx += 1;
        } else {
            arr[merged_idx] = right[right_idx].clone();
            right_idx += 1;
        }
        merged_idx += 1;
    }

    // Copy any remaining elements from the left array
    while left_idx < left.len() {
        arr[merged_idx] = left[left_idx].clone();
        left_idx += 1;
        merged_idx += 1;
    }

    // Copy any remaining elements from the right array
    while right_idx < right.len() {
        arr[merged_idx] = right[right_idx].clone();
        right_idx += 1;
        merged_idx += 1;
    }
}