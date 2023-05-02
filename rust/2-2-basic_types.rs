fn main() {
    ///////////////////// SCALAR TYPES ////////////////////////
    // Rust requires explicitness when it comes to numeric types. One cannot use a u8 for a u32 casually without error.
    // Luckily Rust makes numeric type conversions very easy with the as keyword.

    // BOOLEAN (true or false)
    let bv = true;

    // SIGNED INTEGERS - i8, i16,  i32,  i64, i128 for representing whole numbers that use the '+' or '-' sign
    let x = 12; // by default this is i32

    // UNSIGNED INTEGERS - u8, u16, u32, u64, u128 for representing nonnegative whole numbers
    let guess: u32 = 42;
    let a = 12u8;

    // FLOATING POINT - f32 f64
    let b = 4.3; // by default this is f64
    let c = 4.3f32;

    //////////////////// COMPOUND TYPES ////////////////////

    // TUPLE - (value, value, ...) for passing fixed sequences of values on the stack
    let t = (13, false);
    // Tuples have a fixed length, cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // access a tuple element directly by using a period (.) followed by the value index.
    let five_hundred = x.0;     // 500
    let six_point_four = x.1;   // 6.4
    let one = x.2;  // 1

    // ARRAYS - [value, value, ...] a collection of similar elements with fixed length.
    // every element of an array must have the same type
    let arr = [1, 2, 3, 4, 5];
    // write type of each element, a semicolon, and then the number of elements in the array.
    let arr_fixed: [i32; 5] = [1, 2, 3, 4, 5];
    // // access a array element directly by using square bracket pair,  followed by the value index.
    let first = a[0];   // 1
    let second = a[3];  // 4
    // initialize an array to contain the same value by specifying the initial value,  and the length of the array
    let a = [3; 5];     //  same as [3, 3, 3, 3, 3];

    // STR(string slice) - text with a length known at runtime
    let sentence = "hello world!";

    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}
//  let x: f64 = 3.14159;