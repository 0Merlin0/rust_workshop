use gui::{Grid, Resizable, GuiController};

// Task 1: Create a struct to represent a two-dimensional cellular automaton generation
//         Add an associated function new() to create a new instance from an appropriate
//         representation of the state in arrays, array slices or vectors
//
// Task 2: Implement the Grid trait. It is defined as:
//
//          trait Grid {
//              fn get(&self, x: usize, y: usize) -> bool;
//              fn set(&mut self, x: usize, y: usize, value: bool);
//              fn get_height(&self) -> usize;
//              fn get_width(&self) -> usize;
//          }

// Task 3: Define a method that given an index returns whether that index is true or false
//         in the next generation.
//
//         Any live cell with two or three live neighbours survives.
//         Any dead cell with three live neighbours becomes a live cell.
//         All other cells die/stay dead in the next generation.
//
//         Hint: Casting a bool to an integer type will yield a 0 for false and a 1 for true
//         Remember: The cell itself is not counted, just its 8 neighbours

// Task 4: Define a method that creates a new generation from the current one
//         Check a few generations of a 3-by-3 grid and display each of them using
//         the show_grid method on the gui
//         The function signature is: fn show_grid(state: &[Vec<bool>])
//         (&[Vec<bool>] can be borrowed from a Vec<Vec<bool>>)
//         It blocks until the Continue button is pressed
//         It returns false if the gui has been closed, true otherwise

// Task 5: Implement the Resizable trait. It is defined as:
//         trait Resizable {
//             fn resize(&mut self, width: usize, height: usize);
//         }
//         It should change the size of your object such that get_height() and get_width()
//         return the given width and height afterwards and get() and set() can get and set
//         data within those new dimensions
//         Use the interactive_grid method instead of the show_grid method to get an interactive
//         game of life gui. It takes a mutable reference instead of an immutable one

//         Congratulations! You are done with the workshop. You can try some of the extra
//         challenges if you feel like it!



fn workshop_main(mut gui: GuiController) {
    let state = vec![ vec![true, false, true],
                      vec![true, false, true],
                      vec![true, false, true]];

    // This method should be replaced with show_grid (Task 5)
    // or interactive_grid (Task 6)
    while gui.show_cellular_2d(&state) {
        // This is where we would advance our generation
    }
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
