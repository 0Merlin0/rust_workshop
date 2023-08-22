use gui::{GuiController, Grid, Resizable};

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

    // Wraps around edges
    fn current_at(&self, x: isize, y: isize) -> bool {
        let width = self.width as isize;
        let x = if x < 0 { width + x } else { x };
        let x = if x >= width { x - width} else { x };
        let height = self.height as isize;
        let y = if y < 0 { height + y } else { y };
        let y = if y >= height { y - height} else { y };
        self.get(x as usize, y as usize)
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

    fn next_iteration(&self) -> LifeGeneration {
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

impl Resizable for LifeGeneration {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.state.resize(height, vec![false; width]);
        for v in &mut self.state {
            v.resize(width, false);
        }
    }
}


fn workshop_main(mut gui: GuiController) {
    let mut state = LifeGeneration::new(&[&[false;10],
                      &[false, false, true],
                      &[true, false, true],
                      &[false, true, true],
                      &[],
                      &[],
                      &[]]);
    while gui.show_interactive_grid(&mut state) {
        state = state.next_iteration();
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
