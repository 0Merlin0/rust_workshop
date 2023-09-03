mod grid;
mod gui;
mod board;

use crate::board::Board;
use crate::gui::{DisplayRequest, DisplayResult};
use std::sync::mpsc;

pub trait Grid {
    fn get(&self, x: usize, y: usize) -> bool;
    fn set(&mut self, x: usize, y: usize, value: bool);
    fn get_height(&self) -> usize;
    fn get_width(&self) -> usize;
}

pub trait Resizable {
    fn resize(&mut self, width: usize, height: usize);
}

pub trait MultiGrid {
    fn get(&self, x: usize, y: usize) -> usize;
    fn set(&mut self, x: usize, y: usize, value: usize);
    fn get_state_count(&self) -> usize;
    fn get_colour(&self, state: usize) -> (u8, u8, u8);
    fn get_height(&self) -> usize;
    fn get_width(&self) -> usize;
}

pub struct GuiController {
    sender: mpsc::Sender<DisplayRequest>,
    receiver: mpsc::Receiver<DisplayResult>,
}

impl GuiController {
    pub fn print(&self, s: &str) {
        match self.sender.send(DisplayRequest::Text(String::from(s))) {
            Ok(()) => (),
            _ => std::process::exit(0),
        };

    }

    pub fn show_3x3(&self, state: [[bool; 3]; 3]) {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        for row in state {
            grid.push(Vec::from(row));
        }
        let board = Board::from_2_d(grid);
        if let Ok(()) = self.sender.send(DisplayRequest::DisplayOnce(board)) { };
    }

    pub fn show_grid<T: Grid>(&self, state: &mut T) -> bool {
        let board = Board::from_grid(state);
        match self.sender.send(DisplayRequest::DisplayClickable(board)) {
            Ok(()) => (),
            _ => return false,
        };

        let board = match self.receiver.recv() {
            Ok(DisplayResult::Full(board)) => board,
            _ => return false,
        };
        board.update_grid(state);
        true
    }

    pub fn show_resizable_grid<T: Grid + Resizable>(&self, grid: &mut T) -> bool {

        let board = Board::from_grid(grid);

        match self.sender.send(DisplayRequest::Full(board)) {
            Ok(()) => (),
            _ => return false,
        };

        let board = match self.receiver.recv() {
            Ok(DisplayResult::Full(board)) => board,
            _ => return false,
        };

        board.update_resizable_grid(grid);
        true
    }

    pub fn show_resizable_multigrid<T: MultiGrid + Resizable>(&self, grid: &mut T) -> bool {

        let board = Board::from_multigrid(grid);

        match self.sender.send(DisplayRequest::Full(board)) {
            Ok(()) => (),
            _ => return false,
        };

        let board = match self.receiver.recv() {
            Ok(DisplayResult::Full(board)) => board,
            _ => return false,
        };

        board.update_resizable_multigrid(grid);
        true
    }
}

pub fn show_grid_with_callback<F>(func: F) -> Result<(), eframe::Error> where F: Fn(GuiController), F: Send + 'static {
    gui::init(func)
}
