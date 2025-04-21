use crate::sorting::sorting::{Sorter, SortingAlgorithm};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[derive(Clone)]
pub struct BubbleSort {}
impl Sorter for BubbleSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        while !arr.is_sorted() {
            for i in 0..arr.len() {
                if i != arr.len() - 1 && arr[i] > arr[i + 1] {
                    // swap values
                    arr.swap(i, i + 1);
                }
                std::thread::sleep(Duration::from_millis(1));
                SortingAlgorithm::refresh(&get_arr, &arr);
            }
        }
    }
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
