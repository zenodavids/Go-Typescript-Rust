// Topic: Functions
//
// Program requirements
// * Display your first and last name
//
// Notes:
// * use a function to display your first name
// * use a function to display your last name
// * use the println macro to display messages to the terminal

fn first_name() {
    println!("Chidozie");
}

fn last_name() {
    println!("Ezeanekwe");
}

fn main() {
    first_name();
    last_name();
}
// to run any activity: cargo run --bin filename
// to run any activity and you dont want to show process: cargo run -q --bin filename
