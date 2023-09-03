#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Task 4: Implement the Grid trait for your struct. The trait is defined as:
//
//          pub trait Grid {
//              fn get(&self, x: usize, y: usize) -> bool;
//              fn set(&mut self, x: usize, y: usize, value: bool);
//              fn get_height(&self) -> usize;
//              fn get_width(&self) -> usize;
//          }
//
//         At the moment we have a static 3x3 grid, so get_height an get_width should
//         just return 3.
//         get should return the value at the given position. set should update it.
//
//         Note: get and set will only be called with x and y values smaller than what
//               is returned by get_width and get_height respectively, but to be sure
//               you should add a boundry check before indexing into your array.
//
//         Once the grid implements these methods, we can use the show_grid method on gui.
//         We can call it with a mut reference to our struct (e.g. gui.show_grid(&mut grid))
//         The show_grid function blocks until the gui requests a state update, or the
//         Window is closed.
//         It returns a bool:
//          * true indicates that the GUI Window is still open and requests the state to
//            be updated.
//          * false indicates that the GUI Window has been closed
//
//         Note that you can now press cells in the GUI to flip their state.
//         If set has been implemented correctly, this change should be reflected
//         in your struct.
//         At this point, pressing step should not change what is displayed. If it does,
//         either set or get might be implemented incorrectly.
//         (The program will terminate if an update is requested after workshop_main has
//         returned. You can use a while loop based on the return value of show_grid, if
//         you want it to keep running until the window is closed)

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
