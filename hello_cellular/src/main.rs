use gui::{GuiController};

// Task 1: Create a struct to represent a one-dimensional cellular automaton generation
//         Add an associated function new() to create a new instance from an array, array
//         slice or vector of bools

// Task 2: Define a method that given an index returns whether that index is true or false
//         in the next generation following the following rule:
//
//         | current | 111 | 110 | 101 | 100 | 011 | 010 | 001 | 000 |
//         -----------------------------------------------------------
//         | next    |  0  |  0  |  0  |  1  |  1  |  1  |  1  |  0  |
//
//         Hint: Defining a current_at function to handle edge cases can make this easier
//         Hint: Indexing a vector takes a usize, which cannot be negative. Remember you can
//               cast an isize to a usize if you know it is not negative.

// Task 3: Define a method that creates a new generation from the current one.
//         Check your implementation with a few generations
//         using the show_cellular method on the gui
//         The signature is: fn show_cellular(state: &[bool])
//         (&[bool] can be borrowed from a Vec<bool>)
//         It blocks until the Continue button is pressed
//         It returns false if the gui has been closed, true otherwise

fn workshop_main(mut gui: GuiController) {
    let state = vec![true, false, true];

    while gui.show_cellular(&state) {
        // This is where we would advance our generation
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
