use crate::{grid, Grid, Resizable};
use eframe::egui;

#[derive(Default, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub state: Vec<Vec<bool>>,
}

impl Board {
    pub fn get(&self, x: usize, y: usize) -> bool {
        if y < self.height && x < self.width {
            self.state[y][x]
        } else {
            false
        }
    }
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if y < self.height && x < self.width {
            self.state[y][x] = value;
        }
    }
    pub fn adjust_vector(&mut self) {
        self.state.resize(self.height, vec![false; self.width]);
        for v in &mut self.state {
            v.resize(self.width, false);
        }
    }
    pub fn new(width: usize, height: usize) -> Self {
        let mut state: Vec<Vec<bool>> = Vec::new();
        for _ in 0..height {
            state.push(vec![false; width]);
        }

        Self {
            width,
            height,
            state,
        }
    }
    pub fn from_1_d(vector: Vec<bool>) -> Self {
        Self {
            width: vector.len(),
            height: 1,
            state: vec![vector],
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
                return_value.set(x, y, *item.get(x).unwrap_or(&false));
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
                return_value.set(x, y, grid.get(x, y));
            }
        }
        return_value
    }

    pub fn update_grid<T: Grid + Resizable>(&self, grid: &mut T) {
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

struct SimpleDisplay {
    board: Board,
}

impl SimpleDisplay {
    fn new(board: Board) -> Self {
        Self { board }
    }
}

pub fn show_1d(board: Vec<bool>) -> Result<(), eframe::Error> {
    show(Board::from_1_d(board))
}

pub fn show_2d(board: Vec<Vec<bool>>) -> Result<(), eframe::Error> {
    show(Board::from_2_d(board))
}

pub fn show_grid<T: Grid>(grid: &T) -> Result<(), eframe::Error> {
    show(Board::from_grid(grid))
}

pub fn show(board: Board) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Cellular Automaton",
        options,
        Box::new(|_cc| Box::<SimpleDisplay>::new(SimpleDisplay::new(board))),
    )
}

impl eframe::App for SimpleDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(grid::grid(&mut self.board, false));
        });
    }
}
