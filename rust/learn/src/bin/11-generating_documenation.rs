// documentation comments are written using three slashes "///"" and are used to provide documentation for functions, structs, enums, modules, and other items in your code. These comments are known as "doc comments" and are written in Markdown format.

/// The `Customer` struct represents a customer with an optional age and an email address.
struct Customer {
    /// The age of the customer, represented as an `Option<i32>`. It can be `Some(age)` or `None`.
    age: Option<i32>,
    /// The email address of the customer, represented as a `String`.
    email: String,
}

fn main() {
    // ... (rest of the code remains the same)
}

// After adding the doc comments, you can generate the documentation for your project by running the following command in your project's directory:

// "cargo doc --open"

// "cargo doc" generates the documentation for your project and its dependencies.
// The "--open" flag automatically opens the generated documentation in your default web browser.
