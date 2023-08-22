mod grid;
mod gui;
mod simple_display;

use crate::simple_display::Board;
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

pub fn show_cellular(state: &[bool]) {
    simple_display::show_1d(Vec::from(state)).unwrap();
}

pub fn show_cellular_2d(state: &[Vec<bool>]) {
    simple_display::show_2d(Vec::from(state)).unwrap();
}

pub fn show_grid<T: Grid>(state: &T) {
    simple_display::show_grid(state).unwrap();
}

pub struct GuiController {
    sender: mpsc::Sender<DisplayRequest>,
    receiver: mpsc::Receiver<DisplayResult>,
}

impl GuiController {
    pub fn show_cellular(&mut self, state: &[bool]) -> bool {
        let board = Board::from_1_d(Vec::from(state));

        match self.sender.send(DisplayRequest::Display(board)) {
            Ok(()) => (),
            _ => return false,
        };

        match self.receiver.recv() {
            Ok(DisplayResult::Display(s)) => s,
            _ => false,
        }
    }
    pub fn show_cellular_2d(&mut self, state: &[Vec<bool>]) -> bool {
        let board = Board::from_2_d(Vec::from(state));

        match self.sender.send(DisplayRequest::Display(board)) {
            Ok(()) => (),
            _ => return false,
        };

        match self.receiver.recv() {
            Ok(DisplayResult::Display(s)) => s,
            _ => false,
        }
    }

    pub fn show_grid<T: Grid>(&mut self, state: &T) -> bool {
        let board = Board::from_grid(state);
        match self.sender.send(DisplayRequest::Display(board)) {
            Ok(()) => (),
            _ => return false,
        };

        match self.receiver.recv() {
            Ok(DisplayResult::Display(s)) => s,
            _ => false,
        }
    }

    pub fn show_interactive_grid<T: Grid + Resizable>(&mut self, grid: &mut T) -> bool {

        let board = Board::from_grid(grid);

        match self.sender.send(DisplayRequest::Full(board)) {
            Ok(()) => (),
            _ => return false,
        };

        let board = match self.receiver.recv() {
            Ok(DisplayResult::Full(board)) => board,
            _ => return false,
        };

        board.update_grid(grid);
        true
    }
}

pub fn show_grid_with_callback<F>(func: F) -> Result<(), eframe::Error> where F: Fn(GuiController), F: Send + 'static {
    gui::init(func)
}
