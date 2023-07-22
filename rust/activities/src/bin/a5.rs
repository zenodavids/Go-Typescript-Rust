// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    //  Use a mutable integer variable
    let mut num = 1;

    // Use a loop statement
    loop {
        //  Print the variable within the loop statement
        println!("{:?}", num);

        if num == 4 {
            //  Use break to exit the loop
            break;
        }
        // increment i by 1 until
        num += 1;
    }
    println!(" we done did it!");
}
