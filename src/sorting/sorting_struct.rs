use std::sync::{Arc, Mutex, RwLock};

use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct SortingAlgorithm {
    pub name: String,
    pub algorithm: Box<dyn Sorter>,
    pub description: String,
}
impl SortingAlgorithm {
    pub fn new(name: String, algorithm: Box<dyn Sorter>, description: String) -> Self {
        Self {
            name,
            algorithm,
            description,
        }
    }
    pub fn refresh(rendering_arr: &Arc<RwLock<Vec<usize>>>, array: &[usize]) {
        let mut write_guard = rendering_arr.write().unwrap();
        *write_guard = array.to_owned();
    }
}

pub trait Sorter: Send {
    fn sort(&self, arr: &mut Arc<RwLock<Vec<usize>>>);
    fn clone_box(&self) -> Box<dyn Sorter>;

    fn sort_prepare(
        &self,
        specific_algorithm: &Box<dyn Sorter>,
        arr_rwlock: &Arc<RwLock<Vec<usize>>>,
    ) {
        let mut arr_rwlock_clone = Arc::clone(arr_rwlock);
        let algorithm_mutex = Arc::new(Mutex::new(specific_algorithm.clone()));
        std::thread::spawn(move || {
            let algorithm_locked = algorithm_mutex.lock().unwrap();
            algorithm_locked.sort(&mut arr_rwlock_clone);
        });
    }
}
impl Clone for Box<dyn Sorter> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// This sort is only for debuging
// Shouldn't be used anywhere
#[derive(Clone)]
pub struct DebugSort {}
impl Sorter for DebugSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        while !arr.is_sorted() {
            arr.shuffle(&mut rand::rng());
            SortingAlgorithm::refresh(get_arr, &arr);
        }
    }
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
