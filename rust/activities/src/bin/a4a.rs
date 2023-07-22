// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a boolean variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let he_lives = true;
    match he_lives {
        true => println!("indeed, God lives"),
        false => println!("it's false? Satan, you lie"),
    }
}
