// Topic: Ownership and Borrowing
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct Milk {
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(milk: &Milk) {
    println!("Quantity: {:?}", milk.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(milk: &Milk) {
    println!("Id: {:?}", milk.id);
}

fn main() {
    let milk = Milk { quantity: 5, id: 1 };

    display_quantity(&milk);
    display_id(&milk);
}
