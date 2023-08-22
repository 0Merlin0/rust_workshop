use gui::{Grid, GuiController};

// Task 3: Define a method that given an index returns whether that index is true or false
//         in the next generation.
//
//         Any live cell with two or three live neighbours survives.
//         Any dead cell with three live neighbours becomes a live cell.
//         All other cells die/stay dead in the next generation.
//
//         Hint: Casting a bool to an integer type will yield a 0 for false and a 1 for true
//         Remember: The cell itself is not counted, just its 8 neighbours

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

    fn current_at(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            false
        } else {
            self.get(x as usize, y as usize)
        }
    }

    fn next_at(&self, x: usize, y: usize) -> bool {
        let x = x as isize;
        let y = y as isize;
        let live_neighbours =
            self.current_at(x-1, y-1) as u32 +
            self.current_at(x-1, y) as u32+
            self.current_at(x-1, y+1) as u32+
            self.current_at(x, y-1) as u32+
            self.current_at(x, y+1) as u32+
            self.current_at(x+1, y-1) as u32+
            self.current_at(x+1, y) as u32+
            self.current_at(x+1, y+1) as u32;
        live_neighbours == 3 || (live_neighbours == 2 && self.current_at(x, y))
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
