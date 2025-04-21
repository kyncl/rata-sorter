use crate::sorting::sorting_struct::{Sorter, SortingAlgorithm};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[derive(Clone)]
pub struct QuickSort {}
impl QuickSort {
    pub fn get_pivot_i(
        arr: &mut Vec<usize>,
        low_i: isize,
        high_i: isize,
        get_arr: &mut Arc<RwLock<Vec<usize>>>,
    ) -> isize {
        let pivot = arr[high_i as usize];
        let mut i = low_i - 1;
        for j in low_i..high_i {
            if arr[j as usize] <= pivot {
                i += 1;
                arr.swap(i as usize, j as usize);
                SortingAlgorithm::refresh(get_arr, arr);
                std::thread::sleep(Duration::from_millis(12));
            }
        }
        arr.swap((i + 1) as usize, high_i as usize);
        SortingAlgorithm::refresh(get_arr, arr);
        std::thread::sleep(Duration::from_millis(12));
        i + 1
    }
    pub fn quick_sort(
        arr: &mut Vec<usize>,
        low_i: isize,
        high_i: Option<isize>,
        get_arr: &mut Arc<RwLock<Vec<usize>>>,
    ) {
        let high_i = high_i.unwrap_or_else(|| (arr.len() - 1) as isize);

        if low_i < high_i {
            let pivot_i = QuickSort::get_pivot_i(arr, low_i, high_i, get_arr);
            QuickSort::quick_sort(arr, low_i, Some(pivot_i - 1), get_arr);
            QuickSort::quick_sort(arr, pivot_i + 1, Some(high_i), get_arr);
        }
    }
}
impl Sorter for QuickSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        QuickSort::quick_sort(&mut arr, 0, None, get_arr);
        std::thread::sleep(Duration::from_millis(12));
        SortingAlgorithm::refresh(get_arr, &arr);
    }

    // this must be implemented or it won't work
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
