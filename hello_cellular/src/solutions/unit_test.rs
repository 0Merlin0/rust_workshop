use gui::{GuiController};

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

#[test]
fn next_generation() {
    let cell = CellularGeneration::new(&[true]);
    let cell = cell.next_generation();
    assert_eq!(cell.state, [true, true, true]);
    let cell = cell.next_generation();
    assert_eq!(cell.state, [true, true, false, false, true]);
    let cell = cell.next_generation();
    assert_eq!(cell.state, [true, true, false, true, true, true, true]);
}
