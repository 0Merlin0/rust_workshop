#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Task 1: Create a struct to represent a 3x3 game of life board using a nested  array
//         of bools

struct Board {
    state: [[bool; 3]; 3],
}

fn workshop_main(gui: GuiController) {
    gui.print("Hello World!");
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
