// Task 2: Write a program that prints the sum of all odd numbers between 0 and 10

fn main() {
    let mut sum = 0;
    for counter in 0..10 {
        if counter % 2 != 0 {
            sum += counter;
        }
    }
    println!("{sum}");
}
