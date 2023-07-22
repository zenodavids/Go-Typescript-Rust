// Strings are automatically borrowed

// Use ".to_owned" or "String::from()" to create an owned copy of a string slice
// The borrowed string slice is automatically converted into an owned String

// Use an owned string when storing a struct.
// Structs should own their data for ownership and lifetime management

// this "&str" is best used with functions
fn print(data: &str) {
    // Print the borrowed string slice
    println!("{:?}", data);
}

fn main() {
    // Print a string slice
    print("a string slice"); // The string slice is automatically borrowed

    // Create an owned string using ".to_owned()"
    let owned_string = "owned string".to_owned();
    // Creates an owned copy of the string slice

    // Create an owned string using "String::from()"
    let another_owned = String::from("another");
    // Creates an owned string directly
    // Explanation: The `String::from()` function is used to directly create an owned String. The string slice "another" is passed as an argument to the function, and it returns a new String that owns the allocated memory.

    // Print the owned strings by borrowing a reference to them
    print(&owned_string);
    print(&another_owned);
    // Explanation: References to the owned strings `owned_string` and `another_owned` are passed as arguments to the `print` function. The references are borrowed to match the function's parameter type of `&str`, allowing the function to print the borrowed string slices.
} // The `owned_string` and `another_owned` variables go out of scope here, and their memory is freed automatically
