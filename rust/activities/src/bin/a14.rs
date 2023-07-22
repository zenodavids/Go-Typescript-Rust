// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct Persons {
    age: i32,
    // * The color and name should be stored as a String
    name: String,
    color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Persons {
            name: String::from("Mike"),
            color: String::from("Green"),
            age: 2,
        },
        Persons {
            name: String::from("Bee"),
            color: String::from("White"),
            age: 24,
        },
        Persons {
            name: String::from("John"),
            color: String::from("Red"),
            age: 7,
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            // * The name and colors should be printed using a function
            print(&person.name);
            print(&person.color);
        }
    }
}
