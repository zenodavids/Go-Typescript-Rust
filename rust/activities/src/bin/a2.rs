// Topic: Basic Arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * use the "{:?}" taken in the println macro to display the result.

// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// * Use a function to display the result
fn display(result: i32) {
    // * use the "{:?}" taken in the println macro to display the result.
    println!("{:?}", result);
}
fn main() {
    let result = add(2, 3);
    display(result);
}

// to run any activity: cargo run --bin filename
// to run any activity and you dont want to show process: cargo run -q --bin filename
