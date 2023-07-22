// an "option" is a type that represents the possibility of a value being either "some" value or "none" at all. This is similar to the concept of "null" or "nil" in other programming languages.

// Here's an example code snippet that demonstrates how to use an option in Rust:

// We define a function that takes in an integer and returns an option of an integer
fn divide_by_two(num: i32) -> Option<i32> {
    if num % 2 == 0 {
        // If the number is even, we return "some" value
        Some(num / 2)
    } else {
        // If the number is odd, we return "none"
        None
    }
}

fn main() {
    let num1 = 10;
    let num2 = 5;

    // We call the divide_by_two function on both numbers and print the result
    match divide_by_two(num1) {
        Some(result) => println!("{} divided by two is {}", num1, result),
        None => println!("{} is an odd number", num1),
    }

    match divide_by_two(num2) {
        Some(result) => println!("{} divided by two is {}", num2, result),
        None => println!("{} is an odd number", num2),
    }
}

// In this code snippet, we define a function called divide_by_two that takes in an integer and returns an option of an integer. If the input number is even, the function returns a "some" value that is the input number divided by two. If the input number is odd, the function returns "none".

// In the main function, we call the divide_by_two function on two numbers (num1 and num2) and print the result. We use a match statement to pattern match on the returned option value. If the option is a "some" value, we print the result of the division. If the option is "none", we print a message saying that the input number is odd.
