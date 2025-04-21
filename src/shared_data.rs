use std::sync::{Arc, RwLock};

use rand::Rng;
use ratatui::widgets::ListState;
use terminal_size::{Height, Width, terminal_size};

use crate::sorting::{sort_algorithms::SortList, sorting_struct::SortingAlgorithm};

#[derive(Clone)]
pub struct SharedData {
    pub array: Arc<RwLock<Vec<usize>>>,
    pub info: String,
    pub sorting_algorithms: Vec<SortingAlgorithm>,
    pub tabs: Vec<String>,
    pub tabs_i: usize,
    pub pp_i: ListState,
    pub show_indexes: bool,
    pub debug_mode: bool,
    pub is_sorted: bool,
    pub show_pp: bool,
}
impl SharedData {
    pub fn new() -> Self {
        let sorting_algorithms = SortList::new().list;
        let tabs = ["sort", "reset", "quit"];
        let tabs = tabs.iter().map(|s| s.to_string()).collect();

        let mut pp_i = ListState::default();
        pp_i.select_next();

        Self {
            info: String::new(),
            array: Arc::new(RwLock::new(Self::set_array())),
            sorting_algorithms,
            show_indexes: true,
            debug_mode: false,
            tabs,
            tabs_i: 0,
            is_sorted: false,
            pp_i,
            show_pp: false,
        }
    }
    pub fn set_array() -> Vec<usize> {
        let mut new_array = vec![];
        let mut terminal_width = 1;
        let screen_width_heigth = terminal_size();
        if let Some((Width(w), Height(_h))) = screen_width_heigth {
            terminal_width = w;
        }

        for _i in 0..((terminal_width * 56) / 172) + 1 {
            new_array.push(rand::rng().random_range(10..100));
        }
        new_array
    }
    pub fn reset(&mut self) {
        self.array = Arc::new(RwLock::new(Self::set_array()));
        self.is_sorted = false;
        self.info = "".to_string();
    }

    pub fn get_render_array(&self) -> Vec<(String, u64)> {
        let render_arr = Arc::clone(&self.array).read().unwrap().clone();
        let mut render_result = vec![];
        let mut i = 0;
        for value in render_arr {
            let mut index_name = String::new();
            if self.show_indexes {
                if i < 10 {
                    index_name = String::from("0");
                }
                i += 1;
                index_name.push_str(i.to_string().as_str());
            } else {
                index_name = String::from("--");
            }
            render_result.push((index_name, value as u64));
        }
        render_result
    }

    pub fn tab_next(&mut self) {
        if self.tabs.get(self.tabs_i + 1).is_some() {
            self.tabs_i += 1;
        } else {
            self.tabs_i = 0;
        }
    }
    pub fn tab_previous(&mut self) {
        if self.tabs_i > 0 {
            self.tabs_i -= 1;
        } else {
            self.tabs_i = self.tabs.len() - 1;
        }
    }

    pub fn list_next(&mut self) {
        self.pp_i.select_next();
        if self.pp_i.selected().unwrap_or(0) >= self.sorting_algorithms.len() {
            self.pp_i.select(Some(0));
        }
    }
    pub fn list_previous(&mut self) {
        if self.pp_i.selected().unwrap_or(0) == 0 {
            self.pp_i.select(Some(self.sorting_algorithms.len()));
        }
        self.pp_i.select_previous();
    }
}
