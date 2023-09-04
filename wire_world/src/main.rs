use gui::{GuiController, MultiGrid, Resizable};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Empty,
    Head,
    Tail,
    Wire,
}

impl State {
    fn to_usize(self) -> usize {
        match self {
            State::Empty => 0,
            State::Wire => 1,
            State::Head => 2,
            State::Tail => 3,
        }
    }

    fn from_usize(u: usize) -> Self {
        match u {
            0 => State::Empty,
            1 => State::Wire,
            2 => State::Head,
            3 => State::Tail,
            _ => unreachable!(),
        }
    }

    fn get_colour(&self) -> (u8, u8, u8) {
        match self {
            State::Empty => (0, 0, 0),
            State::Head => (0, 0, 255),
            State::Tail => (255, 0, 0),
            State::Wire => (255, 255, 0),
        }
    }

}

#[derive(Clone)]
struct Row {
    columns: Vec<State>,
}

impl Row {
    fn new() -> Self {
        Self { columns: Vec::new() }
    }

    fn from(slice: &[State]) -> Self {
        Self { columns: Vec::from(slice) }
    }
}

struct WireWorldGeneration {
    state: Vec<Row>,
    width: usize,
    height: usize,
}

impl WireWorldGeneration {
    fn from(slice: &[&[State]]) -> Self {
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
        let mut instance = WireWorldGeneration {
            state,
            width,
            height
        };

        // Make sure all rows are as long
        // as the width
        instance.resize(width, height);

        instance
    }

    // Wraps around edges
    fn is_electron_head(&self, x: isize, y: isize) -> usize {
        let width = self.width as isize;
        let x = if x < 0 { width + x } else { x };
        let x = if x >= width { x - width} else { x };
        let height = self.height as isize;
        let y = if y < 0 { height + y } else { y };
        let y = if y >= height { y - height} else { y };
        (self.get_state(x as usize, y as usize) == State::Head) as usize
    }

    fn next_at(&self, x: usize, y: usize) -> State {
        match self.get_state(x, y) {
            State::Empty => State::Empty,
            State::Head => State::Tail,
            State::Tail => State::Wire,
            State::Wire => {
                let x = x as isize;
                let y = y as isize;
                let adjacent_heads =
                       self.is_electron_head(x - 1, y - 1)
                     + self.is_electron_head(x - 1, y    )
                     + self.is_electron_head(x - 1, y + 1)
                     + self.is_electron_head(x    , y - 1)
                     + self.is_electron_head(x    , y + 1)
                     + self.is_electron_head(x + 1, y - 1)
                     + self.is_electron_head(x + 1, y    )
                     + self.is_electron_head(x + 1, y + 1);
                if adjacent_heads < 3 && adjacent_heads > 0 {
                    State::Head
                } else {
                    State::Wire
                }
            }
        }
    }

    fn get_state(&self, x: usize, y: usize) -> State {
        match &self.state.get(y) {
            Some(v) => *v.columns.get(x).unwrap_or(&State::Empty),
            None => State::Empty,
        }
    }
    fn set_state(&mut self, x: usize, y: usize, value: State) {
        if y < self.height && x < self.state[y].columns.len() {
                self.state[y].columns[x] = value;
        }
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

impl MultiGrid for WireWorldGeneration {
    fn get(&self, x: usize, y: usize) -> usize {
        self.get_state(x, y).to_usize()
    }
    fn set(&mut self, x: usize, y: usize, value: usize) {
        self.set_state(x, y, State::from_usize(value));
    }
    fn get_state_count(&self) -> usize {
        4
    }
    fn get_colour(&self, state: usize) -> (u8, u8, u8) {
        let state = State::from_usize(state);
        state.get_colour()
    }

    fn get_height(&self) -> usize {
        self.height
    }
    fn get_width(&self) -> usize {
        self.width
    }
}

impl Resizable for WireWorldGeneration {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.state.resize(height, Row::from(&vec![State::Empty; width]));
        for v in &mut self.state {
            v.columns.resize(width, State::Empty);
        }
    }
}


fn workshop_main(gui: GuiController) {
    let mut state = WireWorldGeneration::from(&[&[State::Empty;10],
                      &[State::Empty, State::Tail],
                      &[State::Wire, State::Empty, State::Head],
                      &[State::Empty, State::Wire],
                      &[State::Empty, State::Wire],
                      &[State::Empty, State::Wire],
                      &[State::Empty, State::Wire],
                      &[State::Empty, State::Empty, State::Wire, State::Wire, State::Wire],
                      &[],
                      &[]]);
    while gui.show_resizable_multigrid(&mut state) {
        state.next_generation();
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
