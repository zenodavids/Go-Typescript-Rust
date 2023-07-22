// Memory can either be "moved" or "borrowed"

fn main() {
    /////////////////////////
    // Stack and Heap Memory:
    ////////////////////////
    // memory is divided into two main areas: "stack" and "heap". The stack is used for storing "fixed-size data" with a known lifetime, while the heap is used for "dynamically" allocated data with a flexible lifetime.

    // Stack-allocated integer
    let temporary_name = "alice"; // `x` is allocated on the stack

    // Heap-allocated string
    let permanent_name = String::from("Alice"); // `name` is allocated on the heap

    ////////////////////////////////
    // Ownership and Move Semantics:
    ///////////////////////////////

    // Rust enforces a strict ownership model, where each value has a single owner at any given time. Ownership can be transferred through move semantics.

    // Transfer ownership from `source` to `target`
    let source = String::from("Hello");
    let target = source; // Ownership of the string is moved to `target`

    // The following line would cause a compilation error as `source` is no longer valid
    // println!("Source: {}", source);

    ////////////////////////////////
    // Borrowing and References:
    ///////////////////////////////

    // Borrowing allows "temporary" access to a value "without taking ownership". It is achieved using "references (&)" in Rust..

    // Borrow a reference to the `permanent_name` string
    let reference = &permanent_name; // `reference` borrows a reference to the `permanent_name` string above

    // Use the variables...
    println!(
        "this Name: {} stays on the stack temporarily",
        temporary_name
    );
    println!(
        "this Name: {} stays on the heap permanently",
        permanent_name
    );
    println!("Target: {}", target);
    println!("Reference: {}", reference);
}
