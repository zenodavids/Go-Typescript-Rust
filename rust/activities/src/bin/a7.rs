// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color name to print

enum Color {
    RED,
    GREEN,
    BLUE,
}

fn color_name(pick: Color) {
    match pick {
        Color::RED => println!("I will get a Red Car🛻\n for me!"),
        Color::GREEN => println!("A Green and soft waterbed"),
        Color::BLUE => println!("and a Blue jet✈️ for Bee!"),
    }
}

fn main() {
    color_name(Color::RED);
    color_name(Color::GREEN);
    color_name(Color::BLUE);
}
