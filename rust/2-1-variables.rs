fn main() {
    // rust infers the type of x
    let x = 13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // we can't change the variable since we use the keyword 'const'
     const N: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);

/////////////// CHANGING VARIABLES USING THE "mut" KEYBOARD /////////////////////

// variables are immutable by default
// to mutate variables, we use the 'mut' keyword

    // use the 'let' keyword when assigning y to the value 42
    let mut y = 42;
    println!("{}", y);

    // now the is no need to use the 'let' keyword when assigning y
    y = 13;
    println!("{}", y);
}