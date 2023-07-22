// OPTIONAL DATA

/*
 * A type that may be one of two things;
 * Some data of the specified type (Some(variable name)),
 * or nothing (represented by the `None` variant).
 * Used in scenarios where data may not be required or is unavailable, e.g.:
 * - Unable to find something,
 * - Error occurred while processing,
 * - Ran out of items in a list,
 * - Form field not filled out.
 */

// Define a struct Survey with three fields, all of which are optional
struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    // Create an instance of the Survey struct with optional data
    let response = Survey {
        q1: Some(50),
        q2: Some(true),
        q3: Some("A question".to_string()),
    };

    // Match and handle the optional data in the Survey struct
    match response.q1 {
        Some(ans) => println!("Question 1 answer: {:?}", ans), // If q1 is Some, print the answer
        None => println!("q1: no response"), // If q1 is None, print a message indicating no response
    }

    match response.q2 {
        Some(ans) => println!("Question 2 answer: {:?}", ans), // If q2 is Some, print the answer
        None => println!("q2: no response"), // If q2 is None, print a message indicating no response
    }

    match response.q3 {
        Some(ans) => println!("Question 3 answer: {:?}", ans), // If q3 is Some, print the answer
        None => println!("q3: no response"), // If q3 is None, print a message indicating no response
    }

    // Create two instances of the Customer struct
    // The first customer, zee, has an age of Some(31) and an email.
    let zee = Customer {
        age: Some(31),
        email: String::from("zenodavids@zee.com"),
    };

    // The second customer, bee, has no age (None) and an email.
    let bee = Customer {
        age: None,
        email: String::from("zenodavids@zee.com"),
    };

    // Use a match statement to handle the age of customer bee
    match bee.age {
        // If bee's age is Some(age), print the age.
        Some(age) => println!("Customer Bee is {:?} years old", age),
        // If bee's age is None, print a message indicating that the age is not provided.
        None => println!("Customer age not provided"),
    };
}
