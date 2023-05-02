/* 'if', 'match', 'functions', and 'scope blocks' all have a unique way of returning values in Rust.

If the last statement 'in' an 'if', 'match', 'function', or 'scope block' is an expression without a ';', Rust will return it as a value from the block.

This is a great way to create concise logic that returns a value that can be put into a new variable.
*/
fn example() -> i32 {
    let x = 42;
    // Rust's ternary expression
    // if x is lesser than 42, return -1 else return 1
    let v = if x < 42 { -1 } else { 1 };
    // initialize what is returned above to the variable v and print
    println!("from if: {}", v);

    ///////////////////////////////////////////

    let food = "hamburger";
    // like switch in js, if food is hotdog, return "is hotdog"...
    let result = match food {
        "hotdog" => "is hotdog",
        // notice the braces are optional when its just a single return expression
        // acts as the 'default' keyword in switch statement
        _ => "is not hotdog",
    };
    // will print "is not hotdog"
    println!("identifying food: {}", result);

    ////////////////////////////////////////////

    let v = {
        // This scope block lets us get a result without polluting function scope
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    // The idiomatic way to return a value in rust from a function at the end
    v + 4
}

fn main() {
 // prints all the returned values in the example function above
    println!("from function: {}", example());
}
