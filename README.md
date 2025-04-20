<h1>rata-sorter</h1>
<p>
Graphical showcase of sorting algorithms written in Rust.
</p>

![Main](https://github.com/kyncl/rata-sorter/blob/main/showcase/main.gif?raw=true)

<details>
<summary><h2>Showcase</h2></summary>
    
**Bubble sort**|**Selection sort**
|:--:|:--:|
![BubbleSort](https://github.com/kyncl/rata-sorter/blob/main/showcase/bubble.gif?raw=true)|![SelectionSort](https://github.com/kyncl/rata-sorter/blob/main/showcase/selection.gif?raw=true)
**Insertion sort**|**Quick sort**
![InsertionSort](https://github.com/kyncl/rata-sorter/blob/main/showcase/insertion.gif?raw=true)|![QuickSort](https://github.com/kyncl/rata-sorter/blob/main/showcase/quick.gif?raw=true)
**Merge sort**|**Bogo sort**
![MergeSort](https://github.com/kyncl/rata-sorter/blob/main/showcase/merge.gif?raw=true)|![BogoSort](https://github.com/kyncl/rata-sorter/blob/main/showcase/bogo.gif?raw=true)

</details>

## How to install
### Requirements
- git
- cargo
- terminal

```bash
git clone git@github.com:kyncl/rata-sorter.git
cargo build --release
./target/release/rata-sorter
```

## How to create new sort
### Create new file
- Create new file in ```src/sorting/sort_algorithms/``` named after your sorting_algorithm.rs
- In that file you'll need to add this:
  
```rust
use crate::sorting::sorting::{Sorter, SortingAlgorithm};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[derive(Clone)]
pub struct NameOfSort {}
impl Sorter for NameOfSort {
    fn sort(&self, get_arr: &mut Arc<RwLock<Vec<usize>>>) {
        let mut arr = get_arr.read().unwrap().clone();
        while !arr.is_sorted() {
            // modification of arr
            // ...
            
            // It's not needed, but in most cases you won't see anything.
            std::thread::sleep(Duration::from_millis(1));

            // this call when you need to show how array looks
            // basically only needed for the rendering
            SortingAlgorithm::refresh(&get_arr, &arr);
        }
    }

    // this must be implemented or it won't work
    fn clone_box(&self) -> Box<dyn Sorter> {
        Box::new(self.clone())
    }
}
```
- Of course change NameOfSort and fn sort() however you like.

### Mod the file
- In file ```src/sorting/sort_algorithms/mod.rs``` add:

```rust
pub mod sorting_algorithm.rs;
```

### Add into vector
- Same file ```src/sorting/sort_algorithms/mod.rs``` should have definition of SortList struct.
- It contains fn new() with let list: Vec<SortingAlgorithm> variable.
- Add into this list new SortingAlgorithm:
  
```rust
SortingAlgorithm::new(
    "Name of the sort".to_string(), // Name of the sort
    Box::new(NameOfSort {}), // New box with your algorithm
    "Best: {} | Average: {} | Worst: {}".to_string(), // Description.
                                                      // It is recommended to just add 
                                                      // best, average and wort possible time complexity
)
```


