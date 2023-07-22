// Structs, short for "structures", are used to create custom data types that can hold multiple related values. They are fundamental to defining custom data structures in Rust.

// Structs are useful for organizing related data together and providing a convenient way to access and manipulate the data.

struct GroceryItem {
    stock: i32,
    price: f64, // f64 is for decimals numbers
}

fn main() {
    // Create an instance of the GroceryItem struct
    let milk = GroceryItem {
        stock: 10,
        price: 1.99,
    };

    // Access and print the fields of the GroceryItem struct
    println!("Stock: {:?}", milk.stock);
    println!("Price: {:?}", milk.price);
}
