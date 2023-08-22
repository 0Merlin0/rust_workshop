// Task 3: Write a function that takes an array of 5 unsigned 32-bit integers and returns the
//         largest of them

fn largest(array: [u32;5]) -> u32 {
    let mut largest = 0;
    for i in array {
        largest = if largest > i {
            largest
        } else {
            i
        }
    }
    largest
}

fn main() {
    let array = [5, 2, 10, 42, 0];
    let largest = largest(array);
    println!("{largest}");
}
