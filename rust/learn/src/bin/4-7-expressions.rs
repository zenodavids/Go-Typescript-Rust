// Expressions in Rust are code constructs that produce a value. They can be as simple as literals or variable names, or they can involve more complex operations. Rust allows for nesting expressions, which means you can use expressions within other expressions to perform calculations or compose more intricate logic.

// "if" and "match" expressions can be nested
// ! best to not use more than two to three levels of nesting

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    // only admins can access th secret file
    let admin_access_level = Access::Admin;

    let can_access_file = match admin_access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("Only Admin can access: {}", can_access_file);

    ////////////////////
    // Another Example
    ////////////////////

    // Set the time of day as "morning"
    let time_of_day = "morning";
    // Set the weather as "sunny"
    let weather = "sunny";

    // Determine the mood based on time of day and weather
    let mood = {
        // Declare a block to determine activities based on time of day
        let activities = match time_of_day {
            // If it's morning, set activities as "having breakfast"
            "morning" => "having breakfast",
            // If it's afternoon, set activities as "going for a walk"
            "afternoon" => "going for a walk",
            // For any other time, set activities as "relaxing at home"
            _ => "relaxing at home",
        };

        // Check if the weather is sunny
        if weather == "sunny" {
            // If sunny, format the mood string as "feeling happy and [activities]"
            format!("feeling happy and {}", activities)
        } else {
            // If not sunny, format the mood string as "[weather] and [activities]"
            format!("feeling {} and {}", weather, activities)
        }
    };

    // Print the mood for the day
    println!("Today, I am {}", mood);
}
