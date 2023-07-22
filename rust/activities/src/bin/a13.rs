// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    // Use a vector to store 4 numbers
    let numbers = vec![10, 20, 30, 40]; // `numbers` is the owner of the vector

    // Iterate through the vector using a for..in loop
    for number in &numbers {
        // Determine whether to print the number or print "thirty" inside the loop
        match number {
            30 => println!("thirty"),      // If number is 30, print "thirty"
            _ => println!("{:?}", number), // For any other number, print the number itself
        }
    }

    // Use the .len() function to print the number of elements in a vector
    println!("Number of elements: {:?}", numbers.len());
}
