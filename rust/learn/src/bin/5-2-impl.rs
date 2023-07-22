struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    // Associated function to create a new Temperature instance representing freezing temperature
    fn freezing() -> Self {
        // `Self` refers to the type itself, which is `Temperature` in this case
        Self { degrees_f: 32.0 } // Creates a new Temperature instance with degrees_f set to 32.0
    }

    // Associated function to create a new Temperature instance representing boiling temperature
    fn boiling() -> Self {
        // `Self` refers to the type itself, which is `Temperature` in this case
        Self { degrees_f: 212.0 } // Creates a new Temperature instance with degrees_f set to 212.0
    }

    // Method to display the temperature
    fn show_temp(&self) {
        // `&self` is a borrowed reference to an instance of the Temperature struct
        println!("{:?} degrees F", self.degrees_f);
    }
}
/***** difference between &self and Self: *****/

// "&self" is a borrowed reference to an instance of the Temperature struct. It allows us to access the data within the instance without taking ownership.

// "Self" (with a capital "S") is a special keyword that represents the type itself. In this case, Self is equivalent to Temperature.

fn main() {
    // Create a new Temperature instance representing boiling temperature
    let boiling = Temperature::boiling(); // Uses the associated function

    // Display the temperature
    boiling.show_temp(); // Calls the show_temp method on the boiling instance

    // Create a new Temperature instance representing freezing temperature
    let freezing = Temperature::freezing(); // Uses the associated function

    // Display the temperature
    freezing.show_temp(); // Calls the show_temp method on the freezing instance

    // Create a new Temperature instance with a custom temperature
    let hot = Temperature { degrees_f: 99.9 }; // Creates a new Temperature instance directly

    // Display the temperature
    hot.show_temp(); // Calls the show_temp method on the hot instance
}
