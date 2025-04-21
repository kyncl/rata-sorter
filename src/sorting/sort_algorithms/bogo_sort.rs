use rand::seq::SliceRandom;

use crate::sorting::sorting_struct::{Sorter, SortingAlgorithm};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[derive(Clone)]
pub struct BogoSort {}
impl Sorter for BogoSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        while !arr.is_sorted() {
            arr.shuffle(&mut rand::rng());
            std::thread::sleep(Duration::from_millis(1));
            SortingAlgorithm::refresh(get_arr, &arr);
        }
    }

    // this must be implemented or it won't work
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
