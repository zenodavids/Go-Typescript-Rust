//  "generics" are a way to write code that can work with multiple different types. it can be used with different types of data.

// Here's an example code snippet that demonstrates how to use generics:

// We define a function called "print_twice" that takes in a value of any type that implements the "Display" trait
fn print_twice<T: std::fmt::Display>(value: T) {
    println!("{}", value);
    println!("{}", value);
}

fn main() {
    let name = "Alice";
    let age = 30;

    // We call the "print_twice" function with a string and an integer
    print_twice(name);
    print_twice(age);
}

// In this code snippet, we define a function called print_twice that takes in a value of any type that implements the "Display" trait. The "Display" trait is a Rust trait that allows a value to be converted into a string for printing.

// Inside the print_twice function, we print the value twice by calling println! with the {} format specifier. This format specifier works with any type that implements the "Display" trait.

// In the main function, we call the print_twice function with a string (name) and an integer (age). Since both name and age implement the "Display" trait, we can pass them to the print_twice function without any issues.
