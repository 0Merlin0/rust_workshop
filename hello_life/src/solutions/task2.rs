#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Task 2: Create an associated function called "new", that creates an instance of the
//         struct with all false values and an associated function "from" that takes a
//         nested array of bools and returns an instance of the struct

struct Board {
    state: [[bool; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Self { state: [[false; 3]; 3] }
    }

    fn from(state: [[bool; 3]; 3]) -> Self {
        Self { state }
    }
}

fn workshop_main(gui: GuiController) {
    gui.print("Hello World!");
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
