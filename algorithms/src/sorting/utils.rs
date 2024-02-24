// Responsible for misc utils for sorting algorithms

pub fn swap(arr: &mut Vec<i32>, from_index: usize, to_index: usize) {
 let temp = arr[from_index];
 arr[from_index] = arr[to_index];
 arr[to_index] = temp;
}