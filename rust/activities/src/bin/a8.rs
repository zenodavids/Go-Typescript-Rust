// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// Use an enum to create different flavors of drinks
enum Flavor {
    Chocolate,
    Coffee,
    Lemon,
}

// Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    ounce: f64,
}

// Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Chocolate => println!("Chocolate"),
        Flavor::Coffee => println!("Coffee"),
        Flavor::Lemon => println!("Lemon"),
    }
    println!("oz: {:?}", drink.ounce);
}

fn main() {
    // ==========> Create a chocolate drink
    let chocolate = Drink {
        flavor: Flavor::Chocolate,
        ounce: 5.2,
    };

    print_drink(chocolate);

    // ==========> Create a coffee drink
    let coffee = Drink {
        flavor: Flavor::Coffee,
        ounce: 6.0,
    };

    print_drink(coffee);

    // ==========> Create a lemon drink
    let lemon = Drink {
        flavor: Flavor::Lemon,
        ounce: 0.2,
    };

    print_drink(lemon);
}
