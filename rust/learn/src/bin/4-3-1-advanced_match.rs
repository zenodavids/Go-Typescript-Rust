enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    // Example 1: Basic Match
    let num = 3;
    match num {
        3 => println!("three"), // Match the value 3 and print "three"

        // rather than use underscore (_) to match anything else, give it a name.
        // here we replaced the underscore with "other"
        other => println!("Number: {:?}", other), // Match any other value and print it using `{:?}`
    }

    //TODO: Example 2: How to Match with Enum
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("Flat 2"), // Match the specific variant `Flat(2)` and print "Flat 2"
        Discount::Flat(amount) => println!("Flat discount of {:?}", amount), // Match any `Flat` variant and print the associated value if the flat value isn't "2"
        _ => (), // means if none matches, print nothing
    }

    //TODO: Example 3: How to Match with Structs
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        // the ".." means get only the price field, and ignore the rest
        // the first most correct one is matched as the answer.
        Ticket { price: 50, event } => println!("event @ $50 = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
