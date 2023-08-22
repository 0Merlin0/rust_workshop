use gui::{GuiController};
// Task 1: Create a struct to represent a one-dimensional cellular automaton generation
//         Add an associated function new() to create a new instance from an array, array
//         slice or vector of bools

struct CellularGeneration {
    state: Vec<bool>,
}

impl CellularGeneration {
    fn new(slice: &[bool]) -> Self {
        CellularGeneration {
            state: Vec::from(slice),
        }
    }
}

fn workshop_main(mut gui: GuiController) {
    let cell = CellularGeneration::new(&[false, true, true]);

    while gui.show_cellular(&cell.state) {
        // This is where we would advance our generation
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
