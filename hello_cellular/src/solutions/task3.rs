use gui::{GuiController};

// Task 3: Define a method that creates a new generation from the current one.
//         Check your implementation with a few generations
//         using the show_cellular method on the gui
//         The signature is: fn show_cellular(state: &[bool])
//         (&[bool] can be borrowed from a Vec<bool>)
//         It blocks until the Continue button is pressed
//         It returns false if the gui has been closed, true otherwise

struct CellularGeneration {
    state: Vec<bool>,
}

impl CellularGeneration {
    fn new(slice: &[bool]) -> Self {
        CellularGeneration {
            state: Vec::from(slice),
        }
    }

    fn current_at(&self, index: isize) -> bool {
        if index < 0 {
            return false;
        }
        let index = index as usize;
        if index < self.state.len() {
            self.state[index]
        } else {
            false
        }
    }

    fn next_at(&self, index: isize) -> bool {
        let left = self.current_at(index - 2);
        let centre = self.current_at(index - 1);
        let right = self.current_at(index);
        (left && !centre && !right) ||
        (!left && centre && !right) ||
        (!left && !centre && right) ||
        (!left && centre && right)
    }

    fn next_generation(&self) -> Self {
        let mut new_generation = Self {
            state: Vec::new()
        };
        for i in 0..self.state.len() + 2 {
            new_generation.state.push(self.next_at(i as isize));
        }
        new_generation
    }
}

fn workshop_main(mut gui: GuiController) {
    let mut cell = CellularGeneration::new(&[true]);

    while gui.show_cellular(&cell.state) {
        cell = cell.next_generation();
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
