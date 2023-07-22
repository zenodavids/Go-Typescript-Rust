// Topic: Decision making with match
//
// Program requirements:
// * Display "none", "two", "three" or "other" based on whether the value of a variable is 1, 2, 3 or some other number, respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * use an underscore (_) o match any variable

fn main() {
    let num = 2;

    match num {
        // between 1 to 9
        1..=9 => println!("between one through nine"),
        // 2 => println!("two"), uncommenting this will cause an error because "1 to 9 above already has 2 and 3"
        // 3 => println!("three"), uncommenting this will cause an error because "1 to 9 above already has 2 and 3"
        _ => println!("other"),
    }
}
