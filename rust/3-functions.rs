/**
* declare functions using the "fn" keyword.
* must return type unless it is an empty tuple"()"

fn function_name(first_argument: first_argument type, second_argument: second_argument type) -> return_type {
  return 0;
}
*/

// "x: i32, y: i32" simply means x and y are parameters with both i32(integar) type
// " -> " means the function will return something. its return type is i32, hence "-> i32"
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
// assign the add() function to the variable k
let k = add(2, 5)
let n = add(k, 5)

// missing the semi-colon':' is other way to return an expression
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

// return multiple values by returning a "tuple" of values.
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));

    // return a tuple of return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
/*
// Function parameters are typed using the : syntax
fn main() {
// underscore before variable name indicate that the variable is unused.
  let _unused_variable = my_func(10);
}

fn my_func(x: u8) -> i32 {
  x as i32
}
*/
