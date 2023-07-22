// Define an enum representing different types of fruits
#[derive(Debug)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    // We can also associate data with each variant
    // For example, the Strawberry variant includes a u32 value representing its sweetness level
    Strawberry(u32),
}

// Define a function to process and print the type of fruit
fn juice(fruit: Fruit) {
    // We can use the match keyword to handle different variants of the enum
    // In this case, we match against the input `fruit` and execute corresponding code blocks
    match fruit {
        Fruit::Apple => println!("It's an apple!"),
        Fruit::Banana => println!("It's a banana!"),
        Fruit::Orange => println!("It's an orange!"),
        Fruit::Strawberry(sweetness) => {
            // For the Strawberry variant, we extract the sweetness value and print it
            println!("It's a strawberry with sweetness level {}!", sweetness);
        }
    }
}

#[derive(Debug)]
// Define an enum representing different types of discount options
enum PromoDiscount {
    NewUser,
    Holiday(String), // We can associate data with the Holiday variant (a String representing the holiday name)
}

#[derive(Debug)]
enum Discount {
    Percent(f64),         // Represents a percentage discount with a floating-point value
    Flat(i32),            // Represents a flat discount with an integer value
    Promo(PromoDiscount), // Represents a promotional discount with associated data from PromoDiscount enum
    Custom(String),       // Represents a custom discount with a descriptive string
}

fn main() {
    // Calling the juice function with different Fruit variants
    juice(Fruit::Strawberry(8));
    juice(Fruit::Apple);
    juice(Fruit::Banana);
    juice(Fruit::Orange);

    // Creating a discount variable with the Discount enum variants
    let discount = Discount::Percent(20.0);

    // Using match to handle different Discount variants
    match discount {
        Discount::Percent(value) => {
            // For the Percent variant, we extract the floating-point value and print the discount percentage
            println!("Discount: {}%", value);
        }
        Discount::Flat(value) => {
            // For the Flat variant, we extract the integer value and print the flat discount amount
            println!("Discount: ${}", value);
        }
        Discount::Promo(promo) => match promo {
            // For the Promo variant, we extract the inner PromoDiscount variant and match against it
            PromoDiscount::NewUser => {
                // For the NewUser variant, we print a message about the discount for new users
                println!("Discount for new users");
            }
            PromoDiscount::Holiday(name) => {
                // For the Holiday variant, we extract the associated String value and print the holiday discount
                println!("Discount for holiday: {}", name);
            }
        },
        Discount::Custom(description) => {
            // For the Custom variant, we extract the associated String value and print the custom discount
            println!("Custom discount: {}", description);
        }
    }

    // Creating instances of unused enum variants to avoid compiler warnings
    let _unused_promo = Discount::Promo(PromoDiscount::NewUser);
    let _unused_holiday = PromoDiscount::Holiday("Christmas".to_string());
    println!("{:?}", _unused_promo);
    println!("{:?}", _unused_holiday);
}
