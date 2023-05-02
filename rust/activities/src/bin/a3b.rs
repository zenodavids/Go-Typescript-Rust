// Topic: Flow control using if...else
//
// Program requirements:
// * Display ">5", or "=5" based on the value of a variable is > 5, < 5, or == 5, respectively.
//
// Notes:
// * use a variable set to any integer value
// * use an if..else if..else block to determine which message to display
// * use the println macro to display messages to the terminal

fn main() {
    let num = 7;

    if num > 5 {
        println!(">5");
    } else if num < 5 {
        println!("<5")
    } else {
        println!("=5");
    }
}
