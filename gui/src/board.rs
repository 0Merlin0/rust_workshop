use crate::{Grid, Resizable, MultiGrid};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub state: Vec<Vec<usize>>,
    pub states: Vec<(u8, u8, u8)>,
}

impl Board {
    pub fn get(&self, x: usize, y: usize) -> usize {
        if y < self.height && x < self.width {
            self.state[y][x]
        } else {
            0
        }
    }
    pub fn get_bool(&self, x: usize, y:usize) -> bool {
        self.get(x, y) != 0
    }
    pub fn set(&mut self, x: usize, y: usize, value: usize) {
        if y < self.height && x < self.width {
            self.state[y][x] = value;
        }
    }
    pub fn set_bool(&mut self, x: usize, y:usize, value: bool) {
        self.set(x, y, value as usize);
    }
    pub fn adjust_vector(&mut self) {
        self.state.resize(self.height, vec![0; self.width]);
        for v in &mut self.state {
            v.resize(self.width, 0);
        }
    }
    pub fn next(&mut self, x: usize, y: usize) {
        let new_value = (self.get(x, y) + 1) % self.states.len();
        self.set(x, y, new_value);
    }
    pub fn get_colour(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let state = self.get(x, y);
        match self.states.get(state) {
            Some(c) => *c,
            None => (255, 255, 255),
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut state: Vec<Vec<usize>> = Vec::new();
        for _ in 0..height {
            state.push(vec![0; width]);
        }
        let states = vec![(255, 255, 255), (0, 0, 0)];

        Self {
            width,
            height,
            state,
            states,
        }
    }

    pub fn with_states(width: usize, height: usize, states: Vec<(u8, u8, u8)>) -> Self {
        let mut state: Vec<Vec<usize>> = Vec::new();
        for _ in 0..height {
            state.push(vec![0; width]);
        }

        Self {
            width,
            height,
            state,
            states,
        }
    }

    pub fn from_2_d(vector: Vec<Vec<bool>>) -> Self {
        let height = vector.len();
        let mut width = 0;
        for v in &vector {
            width = if width > v.len() { width } else { v.len() }
        }
        let mut return_value = Self::new(width, height);
        for x in 0..width {
            for (y, item) in vector.iter().enumerate().take(height) {
                return_value.set_bool(x, y, *item.get(x).unwrap_or(&false));
            }
        }
        return_value
    }
    pub fn from_grid<T: Grid>(grid: &T) -> Self {
        let height = grid.get_height();
        let width = grid.get_width();
        let mut return_value = Self::new(width, height);
        for y in 0..height {
            for x in 0..width {
                return_value.set_bool(x, y, grid.get(x, y));
            }
        }
        return_value
    }

    pub fn from_multigrid<T: MultiGrid>(grid: &T) -> Self {
        let height = grid.get_height();
        let width = grid.get_width();
        let mut states = Vec::new();
        for state in 0..grid.get_state_count() {
            states.push(grid.get_colour(state));
        }
        let mut return_value = Self::with_states(width, height, states);
        for y in 0..height {
            for x in 0..width {
                return_value.set(x, y, grid.get(x, y));
            }
        }
        return_value
    }

    pub fn update_grid<T: Grid>(&self, grid: &mut T) {
        for x in 0..grid.get_width() {
            for y in 0..grid.get_height() {
                grid.set(x, y, self.get_bool(x, y));
            }
        }
    }

    pub fn update_resizable_grid<T: Grid + Resizable>(&self, grid: &mut T) {
        if self.width != grid.get_width() || self.height != grid.get_height() {
            grid.resize(self.width, self.height);
        }
        for x in 0..grid.get_width() {
            for y in 0..grid.get_height() {
                grid.set(x, y, self.get_bool(x, y));
            }
        }
    }

    pub fn update_resizable_multigrid<T: MultiGrid + Resizable>(&self, grid: &mut T) {
        if self.width != grid.get_width() || self.height != grid.get_height() {
            grid.resize(self.width, self.height);
        }
        for x in 0..grid.get_width() {
            for y in 0..grid.get_height() {
                grid.set(x, y, self.get(x, y));
            }
        }
    }
}
