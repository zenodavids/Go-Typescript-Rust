// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum BoxColor {
    Red,
}

impl BoxColor {
    // Define a method to print the box color
    fn print(&self) {
        match self {
            // Match the BoxColor enum variant
            BoxColor::Red => println!("Red"),
        }
    }
}

// Define a struct to represent the dimensions of a box
struct Dimension {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimension {
    // Define a method to print the box dimensions
    fn print(&self) {
        println!(
            "height: {:?}, width: {:?}, depth: {:?}",
            self.height, self.width, self.depth
        );
    }
}

// * Use a struct to encapsulate the box characteristics
struct Box {
    weight: f64,
    color: BoxColor,
    dimension: Dimension,
}

// * Implement functionality on the box struct to create a new box
impl Box {
    // Define a static method to create a new box
    fn new() -> Self {
        Self {
            weight: 45.0,
            color: BoxColor::Red,
            dimension: Dimension {
                height: 60.04,
                width: 21.05,
                depth: 10.09,
            },
        }
    }

    // Define a method to print the box characteristics
    fn print(&self) {
        // Call the print method of the BoxColor enum variant
        self.color.print();
        // Call the print method of the Dimension struct
        self.dimension.print();
        println!("weight: {:?}", self.weight);
    }
}

// * Implement functionality on the box struct to print the characteristics

fn main() {
    // Create a new box by calling the new method of the Box struct
    let red_box = Box::new();
    // Call the print method of the Box struct to print the box characteristics
    red_box.print();
}
