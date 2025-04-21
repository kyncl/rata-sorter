use crate::sorting::sorting_struct::{Sorter, SortingAlgorithm};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[derive(Clone)]
pub struct MergeSort {}
impl MergeSort {
    fn merge_sort(arr: &mut Vec<usize>, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        if arr.len() <= 1 {
            return;
        }

        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        Self::merge_sort(&mut left, get_arr);
        Self::merge_sort(&mut right, get_arr);
        Self::merge(arr, &left, &right, get_arr);
    }

    pub fn merge(
        arr: &mut [usize],
        left: &[usize],
        right: &[usize],
        get_arr: &mut Arc<RwLock<Vec<usize>>>,
    ) {
        let mut i = 0; // Index for left array
        let mut j = 0; // Index for right array
        let mut k = 0; // Index for merged array

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                arr[k] = left[i];
                i += 1;
            } else {
                arr[k] = right[j];
                j += 1;
            }
            std::thread::sleep(Duration::from_millis(12));
            SortingAlgorithm::refresh(get_arr, arr);
            k += 1;
        }
        while i < left.len() {
            arr[k] = left[i];
            std::thread::sleep(Duration::from_millis(12));
            SortingAlgorithm::refresh(get_arr, arr);
            i += 1;
            k += 1;
        }
        while j < right.len() {
            arr[k] = right[j];
            std::thread::sleep(Duration::from_millis(12));
            SortingAlgorithm::refresh(get_arr, arr);
            j += 1;
            k += 1;
        }
    }
}
impl Sorter for MergeSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        MergeSort::merge_sort(&mut arr, get_arr);
        std::thread::sleep(Duration::from_millis(12));
        SortingAlgorithm::refresh(get_arr, &arr);
    }

    // this must be implemented or it won't work
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
