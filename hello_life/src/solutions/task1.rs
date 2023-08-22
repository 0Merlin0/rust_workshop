use gui::{GuiController};

// Task 1: Create a struct to represent a two-dimensional cellular automaton generation
//         Add an associated function new() to create a new instance from an appropriate
//         representation of the state in arrays, array slices or vectors

struct LifeGeneration {
    state: Vec<Vec<bool>>,
}

impl LifeGeneration {
    fn new(slice: &[&[bool]]) -> Self {
        let mut state: Vec<Vec<bool>> = Vec::new();
        for row in slice {
            state.push(Vec::from(*row));
        }
        LifeGeneration {
            state,
        }
    }
}

fn workshop_main(mut gui: GuiController) {
    let state = LifeGeneration::new(&[&[true, false, true],
                      &[true, false, true],
                      &[true, false, true]]);
    while gui.show_cellular_2d(&state.state) {
        // This is where we would advance our generation
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
