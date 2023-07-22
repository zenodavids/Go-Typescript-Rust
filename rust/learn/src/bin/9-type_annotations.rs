// Topic: Type Annotations

// Functions:
// Functions define reusable blocks of code that can be called with different inputs.

// Enums:
// Enums define a type by enumerating its possible variants.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

// Structs:
// Structs are used to create custom data types by combining different fields.
struct Person {
    name: String,
    age: u32,
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn functions() {
    let result = add(5, 7);
    println!("Result of addition: {}", result);
}

fn main() {
    // Explicit type annotation for an integer
    let age: u32 = 30;

    // Explicit type annotation for a floating-point number
    let pi: f64 = 3.14;

    // Explicit type annotation for a boolean
    let is_rust_awesome: bool = true;

    // Explicit type annotation for a string
    let message: &str = "Hello, Rust!";

    println!("Age: {}", age);
    println!("Pi: {}", pi);
    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Message: {}", message);

    let up: Direction = Direction::Up;
    let down: Direction = Direction::Down;

    println!("Up: {:?}", up);
    println!("Down: {:?}", down);

    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
}
