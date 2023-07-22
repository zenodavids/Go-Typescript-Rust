// find the string "nemo" in an array of strings
fn find_nemo(array: &[&str]) {
    // Start measuring the performance time
    let start_time = std::time::Instant::now();

    // Iterate over each element in the array
    for &element in array {
        // Check if the element is 'nemo'
        if element == "nemo" {
            println!("found NEMO!");
        }
    }

    // Stop measuring the performance time
    let end_time = start_time.elapsed();

    // Print the total performance speed
    println!(
        "Total performance speed was {} milliseconds",
        end_time.as_millis()
    );
}

fn main() {
    let nemo = ["nemo"];

    // Call the find_nemo function with the nemo array
    find_nemo(&nemo);
}
