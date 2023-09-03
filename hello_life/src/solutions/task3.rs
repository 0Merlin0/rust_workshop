#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Task 3: In workshop_main create an instance of your struct using the associated function
//         "new" or "from", that you defined.
//         Then call the show_3x3 method on "gui" (which is a parameter to workshop_main)
//         as a parameter provide the array from your struct (e.g. gui.show_3x3(board.state);)

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
    let board = Board::from(
                [[false, true, false],
                 [false, true, false],
                 [false, true, false]]);
    gui.show_3x3(board.state);
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
