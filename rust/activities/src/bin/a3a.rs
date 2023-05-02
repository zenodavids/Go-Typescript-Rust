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

fn main() {
    let message = true;
    if message == true {
        println!("hello");
    } else {
        println!("goodbye");
    }
}

// to run any activity: cargo run --bin filename
// to run any activity and you dont want to show process: cargo run -q --bin filename
