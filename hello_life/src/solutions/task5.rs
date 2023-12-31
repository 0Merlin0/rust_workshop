#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Task 5: The get method is defined by the trait to take two usize parameters and
//         return a bool.
//         It can be beneficial for the rule calculation to have an additional method
//         that takes signed x and y and an integer return type.
//
//         Implement a current_at method that takes two values x and y of type isize
//         and return the current state of the cell at the given position in the grid as
//         either 1 or 0, instead of true or false.
//
//         Implement a next_at method that takes two values x and y of type isize and
//         returns a bool based on whether the cell will be live or dead in the next
//         generation, according to the rules of the Game of Life.
//         This function should use current_at to get values of the neighboorhood.
//
//         Hints:
//         * Remember anything outside the grid should return 0.
//         * When indexing arrays we need to use a usize typed value. We can cast the
//           isize values to usize using "as" once we have checked that they are not
//           negative.
//         * Casting bool to an integer type (e.g. u32) will yield 1 for true and 0
//           for false.

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

    fn current_at(&self, x: isize, y: isize) -> u32 {
        if x < 0 || y < 0 {
            0
        } else {
            self.get(x as usize, y as usize) as u32
        }
    }

    fn next_at(&self, x: isize, y: isize) -> bool {
        let live_neighbors =
              self.current_at(x - 1, y - 1)
            + self.current_at(x - 1, y    )
            + self.current_at(x - 1, y + 1)

            + self.current_at(x    , y - 1)
            + self.current_at(x    , y + 1)

            + self.current_at(x + 1, y - 1)
            + self.current_at(x + 1, y    )
            + self.current_at(x + 1, y + 1);
        live_neighbors == 3 || (live_neighbors == 2 && self.get(x as usize, y as usize))
    }
}

impl Grid for Board {
    fn get(&self, x: usize, y: usize) -> bool {
        if y < self.state.len() && x < self.state[y].len() {
            self.state[y][x]
        } else {
            false
        }
    }
    fn set(&mut self, x: usize, y: usize, value: bool) {
        if y < self.state.len() && x < self.state[y].len() {
            self.state[y][x] = value;
        }
    }
    fn get_height(&self) -> usize {
        3
    }
    fn get_width(&self) -> usize {
        3
    }
}

fn workshop_main(gui: GuiController) {
    let mut board = Board::from(
                [[false, true, false],
                 [false, true, false],
                 [false, true, false]]);
    while gui.show_grid(&mut board) {
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
