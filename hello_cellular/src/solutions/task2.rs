// Task 2: Define a method that given an index returns wether that index is true or false
//         in the next generation following the following rule:
//
//         | current | 111 | 110 | 101 | 100 | 011 | 010 | 001 | 000 |
//         -----------------------------------------------------------
//         | next    |  0  |  0  |  0  |  1  |  1  |  1  |  1  |  0  |
//
//         Hint: Defining a current_at function to handle edge cases can make this easier
//         Hint: Indexing a vector takes a usize, which cannot be negative. Remember you can
//               cast an isize to a usize if you know it is not negative.

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
}

fn main() {
    let cell = CellularGeneration::new(&[true, true, true]);
    for i in 0..5 {
        println!("{}", cell.next_at(i));
    }
}
