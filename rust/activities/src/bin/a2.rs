// Topic: Basic Arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * use the "{:?}" taken in the println macro to display the result.

fn add(b: i32, z: i32) -> i32 {
    b + z
}
// ============> option 1

// fn display_result() {
//     let c = add(3, 4);
//     println!("{:?}", c);
// }

// fn main() {
//     display_result()
// }

// ===========> option 2

fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let result = add(2, 3);
    display_result(result);
}

// to run any activity: cargo run --bin filename
// to run any activity and you dont want to show process: cargo run -q --bin filename
