// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of an if..else expression to store whether the value is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message to print

// Use a function to print the messages
fn print_message(gt_100: bool) {
    // Use a match expression to determine which message to print
    match gt_100 {
        true => println!("its big"),
        // false => println!("its small"),
        _ => println!("its small"), // also the same as false
    }
}

fn main() {
    // Use a boolean variable set to the result of an if..else expression to store whether the value is > 100 or <= 100
    let value = 100;

    /*let is_gt_100 = if value > 100 {
        println!("its big");
    } else {
        println!("its small");
    }*/

    // will return "false" since the value is 100, not Greater than 100
    let is_gt_100 = value > 100; // replaces the if...else statement

    print_message(is_gt_100);
}
