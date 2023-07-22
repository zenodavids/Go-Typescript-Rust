// Tuples in Rust are another way to group multiple values into a single compound value. Tuples can contain elements of different types, and their length is fixed at compile time.

#[derive(Debug)] //  automatically generate the implementation of the Debug trait for the Access enum, allowing it to be formatted and printed using println!.
enum Access {
    Full,
    Restricted,
}
// ====================== Returning Tuples from Functions
fn get_person_info() -> (String, u32, bool) {
    let name = "Bee".to_string();
    let age = 25;
    let married = true;

    (name, age, married) // Return a tuple
}

fn main() {
    // ======================================================
    // Create a tuple representing a 2D point "x" and "y" ❌
    let point = (3, 5);

    let x = point.0; // Access the first element of the tuple
    let y = point.1; // Access the second element of the tuple
    println!("Bad tuple Coordinates: ({}, {})", x, y);

    // Create a tuple representing a 2D point "x" and "y" ✔️
    let (x, y) = (2, 5);
    println!("Good tuple Coordinates: ({}, {})", x, y);
    // =================================================

    // Create a tuple representing a person's information
    let (name, age, married) = get_person_info(); // Destructure the tuple

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Married: {}", married);

    // from the enum above
    let (employee, access) = ("Jake", Access::Full);
    println!(
        "The new employee {}, already has {:#?} access while i am still {:#?} ?! ",
        employee,
        access,
        Access::Restricted
    );
}
