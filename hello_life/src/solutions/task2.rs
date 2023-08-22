use gui::{Grid, GuiController};

// Task 2: Implement the Grid trait. It is defined as:
//
//          trait Grid {
//              fn get(&self, x: usize, y: usize) -> bool;
//              fn set(&mut self, x: usize, y: usize, value: bool);
//              fn get_height(&self) -> usize;
//              fn get_width(&self) -> usize;
//          }

struct LifeGeneration {
    state: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl LifeGeneration {
    fn new(slice: &[&[bool]]) -> Self {
        let mut state: Vec<Vec<bool>> = Vec::new();
        let height = slice.len();
        let mut width = 0;
        for row in slice {
            state.push(Vec::from(*row));
            width = if width > row.len() {
                width
            } else {
                row.len()
            }
        }
        LifeGeneration {
            state,
            width,
            height
        }
    }
}

impl Grid for LifeGeneration {
    fn get(&self, x: usize, y: usize) -> bool {
        match &self.state.get(y) {
            Some(v) => *v.get(x).unwrap_or(&false),
            None => false,
        }
    }
    fn set(&mut self, x: usize, y: usize, value: bool) {
        if y < self.height && x < self.state[y].len() {
                self.state[y][x] = value;
        }
    }
    fn get_height(&self) -> usize {
        self.height
    }
    fn get_width(&self) -> usize {
        self.width
    }
}

fn workshop_main(mut gui: GuiController) {
    let state = LifeGeneration::new(&[&[true, false, true],
                      &[true, false, true],
                      &[true, false, true]]);
    while gui.show_grid(&state) {
        // This is where we would advance our generation
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
