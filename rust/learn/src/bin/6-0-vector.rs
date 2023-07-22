// Vectors act as arrays in javascript
/*
* vectors contain multiple pieces of "similar" data.
* the "vec!" macro can be used to make vectors
* use "for..in" to iterate through items of a vector
*/

// Define a struct called Test
struct Test {
    score: i32,
}

fn main() {
    // Create a vector called myScores to store Test structs
    let my_scores = vec![
        Test { score: 90 }, // Create a Test struct with score 90 and add it to the vector
        Test { score: 82 }, // Create a Test struct with score 82 and add it to the vector
        Test { score: 93 }, // Create a Test struct with score 93 and add it to the vector
    ]; // `myScores` is the owner of the vector of Test structs

    // Iterate over the vector and print the score of each Test struct
    for test in my_scores {
        println!("score = {:?}", test.score); // Access the score field of each Test struct and print it
    }

    ///////////////////
    // Another Example
    ///////////////////
    // Create an empty vector to store numbers
    let mut numbers: Vec<i32> = Vec::new(); // `numbers` is the owner of the vector

    // Add numbers to the vector
    numbers.push(10); // Push 10 to the vector
    numbers.push(20); // Push 20 to the vector
    numbers.push(30); // Push 30 to the vector

    // Access and print elements of the vector
    println!("First element: {}", numbers[0]); // Access the first element of the vector
    println!("Second element: {}", numbers[1]); // Access the second element of the vector

    // Iterate over the vector and print each element
    for number in &numbers {
        // Borrowing a reference to each element
        println!("Number: {}", number);
    }

    // Get the length of the vector
    let length = numbers.len(); // Get the length of the vector
    println!("Vector length: {}", length);

    // Check if the vector is empty
    let is_empty = numbers.is_empty(); // Check if the vector is empty
    println!("Is vector empty: {}", is_empty);

    // Remove the last element from the vector
    let removed = numbers.pop(); // Remove the last element from the vector and get its value
    println!("Removed element: {:?}", removed);

    // Display the contents of the vector
    println!("Vector: {:?}", numbers);
} // The vector `numbers` goes out of scope here, and the memory is freed automatically
