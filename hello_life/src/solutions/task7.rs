#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Task 7: Add a unit test for your next_generation method.
//         The assert_eq and assert_ne macros require a type to implement
//         both PartialEq and Debug. You can use the derive annotation on
//         your struct to implement them automatically

#[derive(PartialEq, Debug)]
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

    fn next_generation(&mut self) {
        let mut new_state = Self::new();
        for y in 0..self.state.len() {
            for x in 0..self.state[y].len() {
                new_state.set(x, y, self.next_at(x as isize, y as isize));
            }
        }
        *self = new_state;
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
        board.next_generation();
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}

#[test]
fn test_next_generation() {
    let mut grid = Board::from(
               [[false, true, false],
                [false, true, false],
                [false, true, false]]);
    let should_be = Board::from(
               [[false, false, false],
                [true , true , true ],
                [false, false, false]]);
    assert_ne!(grid, should_be);
    grid.next_generation();
    assert_eq!(grid, should_be);
}
