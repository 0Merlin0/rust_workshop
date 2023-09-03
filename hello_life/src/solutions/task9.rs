#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Task 9: Implement the Resizable trait defined as:
//
//         pub trait Resizable {
//             fn resize(&mut self, width: usize, height: usize);
//         }
//
//         After the resize method has been called, the get_width and get_height
//         functions should return the new width and height and calls to get and
//         set within that range should be expected.
//
//         Now we can now use the show_resizable_grid method on gui instead of
//         show_grid

#[derive(Clone, PartialEq, Debug)]
struct Row {
    columns: Vec<bool>,
}

impl Row {
    fn new(width: usize) -> Self {
        Self { columns: vec![false; width] }
    }
    fn from(columns: &[bool]) -> Self {
        Self { columns: Vec::from(columns) }
    }
    fn len(&self) -> usize {
        self.columns.len()
    }
    fn get(&self, column: usize) -> bool {
        if column < self.columns.len() {
            self.columns[column]
        } else {
            false
        }
    }
    fn set(&mut self, column: usize, value: bool) {
        if column < self.columns.len() {
            self.columns[column] = value
        }
    }
    fn resize(&mut self, width: usize) {
        self.columns.resize(width, false);
    }
}

#[derive(PartialEq, Debug)]
struct Board {
    state: Vec<Row>,
    width: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        let mut state: Vec<Row> = Vec::new();
        for _ in 0..height {
            state.push(Row::new(width));
        }
        Self { state, width }
    }

    fn from(rows: &[Row]) -> Self {
        let mut width = 0;
        for row in rows {
            width = if width > row.len() {
                width
            } else {
                row.len()
            }
        }
        Self { state: Vec::from(rows), width }
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
        let mut new_state = Self::new(self.get_width(), self.get_height());
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
            self.state[y].get(x)
        } else {
            false
        }
    }
    fn set(&mut self, x: usize, y: usize, value: bool) {
        if y < self.state.len() && x < self.state[y].len() {
            self.state[y].set(x, value);
        }
    }
    fn get_height(&self) -> usize {
        self.state.len()
    }
    fn get_width(&self) -> usize {
        self.width
    }
}

impl Resizable for Board {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.state.resize(height, Row::new(width));
        for row in &mut self.state {
            row.resize(width);
        }
    }

}

fn workshop_main(gui: GuiController) {
    let mut board = Board::new(10, 10);
    while gui.show_resizable_grid(&mut board) {
        board.next_generation();
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}

#[test]
fn test_next_generation() {
    let mut grid = Board::from(
               &[Row::from(&[false, true, false]),
                 Row::from(&[false, true, false]),
                 Row::from(&[false, true, false])]);
    let should_be = Board::from(
               &[Row::from(&[false, false, false]),
                 Row::from(&[true , true , true ]),
                 Row::from(&[false, false, false])]);
    assert_ne!(grid, should_be);
    grid.next_generation();
    assert_eq!(grid, should_be);
}
