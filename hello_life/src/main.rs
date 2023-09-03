#[allow(unused_imports)]
use gui::{Grid, Resizable, GuiController};

// Implement your solution in the bottom of this file
// Keep the main function as it is and use workshop_main
// instead
//
// Solutions can be found in the solutions for each task
// can be found in the solutions folder as separate files.
//
// --- Slide 15
//
// Task 1: Create a struct to represent a 3x3 game of life board using a nested  array
//         of bools
//
// Task 2: Create an associated function called "new", that creates an instance of the
//         struct with all false values and an associated function "from" that takes a
//         nested array of bools and returns an instance of the struct
//
// Task 3: In workshop_main create an instance of your struct using the associated function
//         "new" or "from", that you defined.
//         Then call the show_3x3 method on "gui" (which is a parameter to workshop_main)
//         as a parameter provide the array from your struct (e.g. gui.show_3x3(board.state);)
//
//
// --- Slide 18
//
// Task 4: Implement the Grid trait for your struct. The trait is defined as:
//
//          pub trait Grid {
//              fn get(&self, x: usize, y: usize) -> bool;
//              fn set(&mut self, x: usize, y: usize, value: bool);
//              fn get_height(&self) -> usize;
//              fn get_width(&self) -> usize;
//          }
//
//         At the moment we have a static 3x3 grid, so get_height an get_width should
//         just return 3.
//         get should return the value at the given position. set should update it.
//
//         Note: get and set will only be called with x and y values smaller than what
//               is returned by get_width and get_height respectively, but to be sure
//               you should add a boundry check before indexing into your array.
//
//         Once the grid implements these methods, we can use the show_grid method on gui.
//         We can call it with a mut reference to our struct (e.g. gui.show_grid(&mut grid))
//         The show_grid function blocks until the gui requests a state update, or the
//         Window is closed.
//         It returns a bool:
//          * true indicates that the GUI Window is still open and requests the state to
//            be updated.
//          * false indicates that the GUI Window has been closed
//
//         Note that you can now press cells in the GUI to flip their state.
//         If set has been implemented correctly, this change should be reflected
//         in your struct.
//         At this point, pressing step should not change what is displayed. If it does,
//         either set or get might be implemented incorrectly.
//         (The program will terminate if an update is requested after workshop_main has
//         returned. You can use a while loop based on the return value of show_grid, if
//         you want it to keep running until the window is closed)
//
//
// Task 5: The get method is defined by the trait to take two usize parameters and
//         return a bool.
//         It can be beneficial for the rule calculation to have an additional method
//         that takes signed x and y and an integer return type.
//
//         Implement a current_at method that takes two values x and y of type isize
//         and return the current state of the cell at the given position in the grid as
//         either 1 or 0, instead of true or false.
//
//         Implement a next_at method that takes two values x and y of type isize and
//         returns a bool based on whether the cell will be live or dead in the next
//         generation, according to the rules of the Game of Life.
//         This function should use current_at to get values of the neighboorhood.
//
//         Hints:
//         * Remember anything outside the grid should return 0.
//         * When indexing arrays we need to use a usize typed value. We can cast the
//           isize values to usize using "as" once we have checked that they are not
//           negative.
//         * Casting bool to an integer type (e.g. u32) will yield 1 for true and 0
//           for false.
//
// Task 6: Implement a method next_generation that takes a mutable self reference and
//         advances it to the next generation according to the game of life rules.
//         Then loop as long as show_grid returns true and call your method in
//         the loop body.
//
//         Hints:
//         * Since each new cell state depends on the current state of several cells,
//           you cannot do this in place, but need to create a new array.
//
// --- Slide 22
//
// Task 7: Add a unit test for your next_generation method.
//         The assert_eq and assert_ne macros require a type to implement
//         both PartialEq and Debug. You can use the derive annotation on
//         your struct to implement them automatically
//
// Task 8: Move from fixed size arrays in your struct to using vectors.
//         You can use nested vectors directly, or create a new type "Row"
//         to give it slightly more structure, or organize it however you
//         want, as long as the Grid trait methods work.
//         Some standard functions like Vec::from will attempt to clone types.
//         If you are going to use a custom type for Rows, it can be a good
//         idea to derive the Clone trait on it, to make these operations work.
//
//         For the "new" function you can now take an initial size parameter.
//         (or width and height, if you prefer a non-square board)
//         For the "from" function you can now take slices instead of arrays to
//         allow initialising grids of different sizes
//         Now that our grid can have different sizes, we need to make sure the
//         get_width and get_height parameters are no longer hard coded.
//
//
// Task 9: Implement the Resizable trait defined as:
//
//         pub trait Resizable {
//             fn resize(&mut self, width: usize, height: usize);
//         }
//
//         After the resize method has been called, the get_width and get_height
//         functions should return the new width and height and calls to get and
//         set within that range should be expected.
//
//         Now we can now use the show_resizable_grid method on gui instead of
//         show_grid
//
// --- Slide 25
//
// Task 10: Use the methods get and get_mut instead of [] to access values in a vector.
//          Use a match expression to handle the returned values.
//

fn workshop_main(gui: GuiController) {
    gui.print("Hello World!");
}

fn main() {
    gui::show_grid_with_callback(workshop_main).unwrap();
}
