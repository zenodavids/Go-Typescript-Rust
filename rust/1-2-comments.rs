fn main() {
    // This is an example of a line comment and the recommended comment style

    /* 
     * This is another type of comment, a block comment. 
     * block comments are extremely useful for temporarily disabling
     * chunks of code. /* Block comments can be /* nested, */ */
     */

    // You can manipulate expressions more easily with block comments
    // than with line comments.
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}