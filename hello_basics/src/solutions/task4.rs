// Task 4: Define a struct with three integer values. Additionally define two functions:
//         One that takes a tuple of three integers and returns an instance of the struct
//         Another that takes an instance of the struct and returns a tuple of three integers

struct MyTuple {
    first: i32,
    second: i32,
    third: i32,
}

fn from_tuple(tuple: (i32, i32, i32)) -> MyTuple {
    MyTuple { first: tuple.0, second: tuple.1, third: tuple.2 }
}

fn to_tuple(my_tuple: MyTuple) -> (i32, i32, i32) {
    (my_tuple.first, my_tuple.second, my_tuple.third)
}

fn main() {
    let my_tuple = from_tuple((1, 2, 5));
    let tuple = to_tuple(my_tuple);
    let (first, second, third) = tuple;
    println!("({first}, {second}, {third})");
}
