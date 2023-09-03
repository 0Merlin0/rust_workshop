// ---- Variables

// Use let keyword to define variable
let variable_name = 8;

// Use : to define type of variable explicitly
let variable_name: u8 = 8;

// Use mut to define a mutable variable
let mut variable_name = 8;

// Use const to define a compile time constant
const PI = 3.14159;

// ---- Basic types

// i - sized integers
// Number defines number of bits
// Avaiblable types: i8, i16, i32, i64, i128, isize
let variable_name: i64 = -12345

// u - unsized integers
// Number defines number of bits
// Avaiblable types: u8, u16, u32, u64, u128, usize
let variable_name: u64 = 12345

// f - floating point
// Number defines number of bits
// Available types: f32, f64
let variable_name: f32 = 3.14159;

// bool - boolean value
// Either true or false
let variable_name: bool = true;

// char - holds a unicode code point
let variable_name: char = '💩';

// unit type - Written as ()
// Basically void
let variable_name: () = ();

// Casting
// Use as to cast a value to a different type
let variable_name: i64 = 10;
let variable_name: u64 = variable_name as u64;

// tuple - Holds multiple values of different types
// comma-separated list of types surrounded by ()
let variable_name: (i32, u32, f32) = (-1, 10, -1.2345);

// access tuple elements using a dot and a
// 0-based index
let variable_name = ("ten", 20, 30.03);
let second_element = variable_name.1;

// get tuple contents with destructuring
let variable_name = ("ten", 20, 30.03);
let (first, second, third) = variable_name;

// array - Holds multiple values of the same type
// type and size separated by a semi-colon and surrounded by []
let variable_name: [i32; 5] = [1, 2, 3, 4, 5]

// access array elements using a 0-based index
// surrounded by []
let variable_name = [10, 20, 30];
let second_element = variable_name[1];

// get array contents with destructuring
let variable_name = [10, 20, 30];
let [first, second, third] = variable_name;


// ---- Custom Types

// Define using the struct keword

struct Building {
    street: String,
    number: u32,
    zip_code: u32,
}

// Initialising

let building = Building {
    street: String::from("Baker Street"),
    number: 1,
    zip_code: 420,
}

// Drop field name when using a variable of the same name

let street = String::from("Baker Street");
let zip_code = 420;
let building = Building {
    street,
    number: 1,
    zip_code,
}


// ---- Control Flow

// If
// Using if keyword, a condition and a {}-block
if count > 10 {
    println!("That's too much, man!");
}

// Else-If

if count > 10 {
    println!("That's too much, man!");
} else if count < 2 {
    println!("That ain't enough, man!");
}

// Else

if count > 10 {
    println!("That's too much, man!");
} else if count < 2 {
    println!("That ain't enough, man!");
} else {
    println!("Thanks, man!");
}

// Returning a value from if

let first = 10;
let second = 12;
let third = 15;

let max = if first > second && first > third {
    first
} else if second > first && second > third {
    second
} else {
    third
}

// loop

let mut counter = 0;
loop {
    println!("{counter}");
    if counter == 5 {
        break;
    }
    counter += 1;
}

// while

let mut counter = 0;
while counter < 5 {
    println!("{counter}");
    counter += 1;
}

// for

for counter in 0..5 {
    println!("{counter}");
}

// for with mut

for mut counter in 0..5 {
    counter += 10;
    println!("{counter}"); // 10, 11, 12, 13, 14
}

// ---- Functions

// Define without parameters

fn function() {
    println!("I'm a function");
}

// Call without arguments

function();

// Define with parameters

fn sum(first: u32, second: u32) {
    let sum = first + second;
    println!("{sum}");
}

// Call with arguments

sum(1, 2);


// Define with return value

fn sum(first: u32, second: u32) -> u32 {
    first + second
}


// ---- Associated functions

// Define associated function

impl Person {
    fn girl(name: String) -> Self {
        Self {
            name,
            gender: String::from("female")
        }
    }
}

// Call associated function

let martha = Person::girl(String::from("Martha"));

// Define method

impl Person {
    fn greet(self) {
        println("Hello, {}", self.name);
    }
}

// Calling method

martha.greet();


// Borrowing

// Define reference type using &

fn print_size(string: &String) {
    println!("{}", string.len());
}

// Borrow a value using &

let message = "Hi there".to_string();
print_size(&message);

// Define mutable reference type using &mut

fn change(string: &mut String) {
    string.push_str(" is cool.");
}

// Borrow a value mutably using &

let mut name = "King Julian".to_string();
change(&mut message);

// Dereference reference using * to assign to it

let mut string = "Harald".to_string();
let reference = &mut string;
*reference = String::from("Haakon");

// Borrow slices using range

let name: &str = "Harald";
print_slice(&name[1..4]);

// ---- Traits

// Defining trait
trait Addressable {
    fn get(&self, x: usize, y: usize) -> bool;
    fn set(&mut self, x:usize, y: usize, value: bool);
    fn print_at(&self, x:usize, y:usize) {
        println!("{}", self.get(x, y));
    }
}

// Implementing trait for type
impl Addressable for Grid {
    fn get (&self, x: usize, y: usize) -> bool {
        self.state[x][y]
    }
    fn set (&mut self, x: usize, y: usize, value: bool) {
        self.state[x][y] = value;
    }
}

// ---- Tests

// define test
#[test]
fn test_function() {
}

// Use assert macros to test conditions
fn test_function() {
    let a = 10;
    assert_eq!(a, 10);
    assert!(true);
}

// ---- Deriving traits

// Derive a trait
#[derive(Copy)]
enum State {
    Alive,
    Dead,
}

// Derive multiple trait
#[derive(Clone, Copy, PartialEq)]
enum State {
    Alive,
    Dead,
}


// ---- Vectors

// Initialise vector
let numbers: Vec<i32> = Vec::new();
let mut primes = vec![2, 3, 5, 7, 11, 13];
let zeros = vec![0; 15];

// Add to vector using push
primes.push(17);

// Access values
primes[0] = 1;
println!("{}", primes[2]);

// Get size of vector using len
println!("{}", primes.len());


// ---- Enums

// Without contained values
enum State {
    Alive,
    Dead,
}

// With contained values
enum State {
    Alive(i32),
    Dead(i32, String),
    Unknown{ details: String },
}

// Pattern matching using match
match s {
    State::Alive(1) => println!("Alive for 1 iteration"),
    State::Alive(a) => println!("Alive for {a} iterations"),
    State::Dead(1, s) => println!("Dead for 1 iteration.
                                  Died because of {s}"),
    State::Dead(a, s) => println!("Dead for {a} iterations.
                                  Died because of {s}"),
    State::Unknown{ details: b } => println!("{b}"),
}

