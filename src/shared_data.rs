use std::sync::{Arc, Mutex, RwLock};

use crate::sorting::sorting::{Sorter, SortingAlgorithm, TestSort};

#[derive(Clone)]
pub struct SharedData {
    pub array: Arc<RwLock<Vec<usize>>>,
    pub info: String,
    pub sorting_algorithms: Vec<SortingAlgorithm>,
}
impl SharedData {
    pub fn new() -> Self {
        let sorting_algorithms = vec![SortingAlgorithm::new(
            "penis".to_string(),
            Box::new(TestSort::new()),
        )];

        Self {
            info: String::new(),
            array: Arc::new(RwLock::new(vec![
                21, 11, 43, 76, 84, 99, 12, 54, 11, 43, 76, 84, 99, 12, 54, 11, 43, 76, 84, 99, 12,
                54, 11, 43, 76, 84, 99, 12, 54, 11, 43, 76, 84, 99, 12, 54, 11, 43, 76, 84, 99, 12,
            ])),
            sorting_algorithms,
        }
    }
}
