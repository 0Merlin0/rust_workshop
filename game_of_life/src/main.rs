use gui::{GuiController, Grid, Resizable};

#[derive(Clone)]
struct Row {
    columns: Vec<bool>,
}

impl Row {
    fn new() -> Self {
        Self { columns: Vec::new() }
    }

    fn from(slice: &[bool]) -> Self {
        Self { columns: Vec::from(slice) }
    }
}

struct LifeGeneration {
    state: Vec<Row>,
    width: usize,
    height: usize,
}

impl LifeGeneration {
    fn from(slice: &[&[bool]]) -> Self {
        let mut state: Vec<Row> = Vec::new();
        let height = slice.len();
        let mut width = 0;
        for row in slice {
            state.push(Row::from(row));
            width = if width > row.len() {
                width
            } else {
                row.len()
            }
        }
        let mut instance = LifeGeneration {
            state,
            width,
            height
        };
        instance.resize(width, height);
        instance
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

    fn current_at_u32(&self, x: isize, y: isize) -> u32 {
        self.current_at(x, y) as u32
    }

    fn next_at(&self, x: usize, y: usize) -> bool {
        let x = x as isize;
        let y = y as isize;
        let live_neighbours =
            self.current_at_u32(x - 1, y - 1) +
            self.current_at_u32(x - 1, y    ) +
            self.current_at_u32(x - 1, y + 1) +

            self.current_at_u32(x    , y - 1) +
            self.current_at_u32(x    , y + 1) +

            self.current_at_u32(x + 1, y - 1) +
            self.current_at_u32(x + 1, y    ) +
            self.current_at_u32(x + 1, y + 1);

        live_neighbours == 3 || (live_neighbours == 2 && self.current_at(x, y))
    }

    fn next_generation(&mut self) {
        let mut state: Vec<Row> = Vec::new();
        let height = self.height;
        let width = self.width;

        for y in 0..height {
            state.push(Row::new());
            for x in 0..width {
                state[y].columns.push(self.next_at(x, y));
            }
        }
        *self = Self { state, width, height };
    }
}

impl Grid for LifeGeneration {
    fn get(&self, x: usize, y: usize) -> bool {
        match &self.state.get(y) {
            Some(v) => *v.columns.get(x).unwrap_or(&false),
            None => false,
        }
    }
    fn set(&mut self, x: usize, y: usize, value: bool) {
        if y < self.height && x < self.state[y].columns.len() {
                self.state[y].columns[x] = value;
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
        self.state.resize(height, Row::from(&vec![false; width]));
        for v in &mut self.state {
            v.columns.resize(width, false);
        }
    }
}


fn workshop_main(gui: GuiController) {
    let mut state = LifeGeneration::from(&[&[false;10],
                      &[false, false, true],
                      &[true, false, true],
                      &[false, true, true],
                      &[],
                      &[],
                      &[]]);
    while gui.show_resizable_grid(&mut state) {
        state.next_generation();
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
