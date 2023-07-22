// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
// =====================================================================
// * Use an enum for the tickets with data associated with each variant
// * Tickets can be Backstage, Vip, and Standard
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(50.0, String::from("Zee")),
        Ticket::Vip(70.0, String::from("Bee")),
        Ticket::Standard(100.0),
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in &tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Price: {:?} Holder: {:?}", price, holder)
            }
            Ticket::Vip(price, holder) => {
                println!("Price: {:?} Holder: {:?}", price, holder)
            }
            Ticket::Standard(price) => println!("Price: {:?}", price),
        }
    }
}
