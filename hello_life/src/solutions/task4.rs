use gui::{Grid, GuiController};

// Task 4: Define a method that creates a new generation from the current one
//         Check a few generations of a 3-by-3 grid and display each of them using
//         the show_grid method on the gui
//         The function signature is: fn show_grid(state: &[Vec<bool>])
//         (&[Vec<bool>] can be borrowed from a Vec<Vec<bool>>)
//         It blocks until the Continue button is pressed
//         It returns false if the gui has been closed, true otherwise

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

    fn next_generation(&self) -> LifeGeneration {
        let mut state: Vec<Vec<bool>> = Vec::new();
        let height = self.height;
        let width = self.width;

        for y in 0..height {
            state.push(Vec::new());
            for x in 0..width {
                state[y].push(self.next_at(x, y));
            }
        }
        Self { state, width, height }
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
    let mut state = LifeGeneration::new(&[&[false, true, false],
                      &[false, true, false],
                      &[false, true, false]]);
    while gui.show_grid(&state) {
        state = state.next_generation();
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
