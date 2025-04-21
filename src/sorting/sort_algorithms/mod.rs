use bogo_sort::BogoSort;
use bubble_sort::BubbleSort;
use insertion_sort::InsertionSort;
use merge_sort::MergeSort;
use quick_sort::QuickSort;
use selection_sort::SelectionSort;

use super::sorting_struct::SortingAlgorithm;

pub mod bogo_sort;
pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

pub struct SortList {
    pub list: Vec<SortingAlgorithm>,
}
impl SortList {
    pub fn new() -> Self {
        let list = vec![
            SortingAlgorithm::new(
                "Bubble sort".to_string(),
                Box::new(BubbleSort {}),
                "Best: O(n) | Average: O(n²) | Worst: O(n²)".to_string(),
            ),
            SortingAlgorithm::new(
                "Selection sort".to_string(),
                Box::new(SelectionSort {}),
                "Best: O(n²) | Average: O(n²) | Worst: O(n²)".to_string(),
            ),
            SortingAlgorithm::new(
                "Insertion sort".to_string(),
                Box::new(InsertionSort {}),
                "Best: O(n) | Average: O(n²) | Worst: O(n²)".to_string(),
            ),
            SortingAlgorithm::new(
                "Quick sort".to_string(),
                Box::new(QuickSort {}),
                "Best: O(n log n) | Average: O(n log n) | Worst: O(n²)".to_string(),
            ),
            SortingAlgorithm::new(
                "Merge sort".to_string(),
                Box::new(MergeSort {}),
                "Best: O(n log n) | Average: O(n log n) | Worst: O(n log n)".to_string(),
            ),
            SortingAlgorithm::new(
                "Bogo sort".to_string(),
                Box::new(BogoSort {}),
                "Best: O(n) | Average: O((n!)n) | Worst: O(infty)".to_string(),
            ),
        ];
        Self { list }
    }
}
