fn main() {
    let x = 42;
    if x < 42 {
        println!("less than 42");
        // we can have multiple 'if/else'
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }

    ////// Using if in a let Statement ///////
    let condition = true;
    // variable will be bound to a value based on the outcome of the if expression which is '5' in this case.
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
