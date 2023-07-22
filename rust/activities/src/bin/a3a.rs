// Topic: Flow Control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display.
// * use the println macro to display messages to the terminal

// Enums - helps working with data easy
enum Direction {
    // the values in an enum code block are called "variants"
    Up,
    Down,
    Left,
    Right,
}

fn compass(direct: Direction) {
    match direct {
        Direction::Up => println!("Compass points UP"),
        Direction::Down => println!("Compass points Down"),
        Direction::Left => println!("Compass points Left"),
        Direction::Right => println!("Compass points Right"),
    }
}

fn main() {
    compass(Direction::Up);
    compass(Direction::Down);
    compass(Direction::Left);
    compass(Direction::Right);
}

// fn main() {
//     // * Displays a message based on the value of a boolean variable
//     let message = true;
//     // * When the variable is set to true, display "hello"
//     if message == true {
//         println!("hello");
//     // * When the variable is set to false, display "goodbye"
//     } else {
//         println!("goodbye");
//     }
// }

// to run any activity: cargo run --bin filename
// to run any activity and you dont want to show process: cargo run -q --bin filename
