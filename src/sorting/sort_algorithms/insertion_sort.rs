use crate::sorting::sorting::{Sorter, SortingAlgorithm};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[derive(Clone)]
pub struct InsertionSort {}
impl Sorter for InsertionSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1); // Swap the element backward until it is in the correct position
                j -= 1;
                std::thread::sleep(Duration::from_millis(1));
                SortingAlgorithm::refresh(&get_arr, &arr);
            }
        }
    }

    // this must be implemented or it won't work
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
