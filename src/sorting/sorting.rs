use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use rand::{Rng, rng, seq::SliceRandom};

#[derive(Clone)]
pub struct SortingAlgorithm {
    pub name: String,
    pub algorithm: Box<dyn Sorter>,
}
impl SortingAlgorithm {
    pub fn new(name: String, algorithm: Box<dyn Sorter>) -> Self {
        Self { name, algorithm }
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
            /* let mut arr = {
                let write_guard = arr_rwlock_clone.write().unwrap();
                write_guard.clone()
            }; */
            // algorithm_locked.sort(&mut arr);
            algorithm_locked.sort(&mut arr_rwlock_clone);
        });
    }
}
impl Clone for Box<dyn Sorter> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct TestSort {}
impl TestSort {
    pub fn new() -> Self {
        Self {}
    }
}
impl Sorter for TestSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        while !arr.is_sorted() {
            arr.shuffle(&mut rand::rng());

            let mut write_guard = get_arr.write().unwrap();
            *write_guard = arr.clone();
        }
    }
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
