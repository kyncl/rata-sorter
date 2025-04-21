use crate::sorting::sorting_struct::{Sorter, SortingAlgorithm};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[derive(Clone)]
pub struct SelectionSort {}
impl Sorter for SelectionSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        for offset in 0..arr.len() {
            let mut smallest_i = offset;
            for i in (offset + 1)..arr.len() {
                if arr[i] < arr[smallest_i] {
                    smallest_i = i;
                }
            }
            arr.swap(offset, smallest_i);
            std::thread::sleep(Duration::from_millis(10));
            SortingAlgorithm::refresh(get_arr, &arr);
        }
    }

    // this must be implemented or it won't work
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
