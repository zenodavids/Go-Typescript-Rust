// Rust has three kinds of loops: 'loop', 'while', and 'for'.
fn main() {
    //========================= loop
    fn loop(){
        //! always set to "mut"(mutable) because it will always be changing as it iterates
    let mut x = 0;
    loop {
 
        // once x gets to 42...
        if x == 42 {
            // ... escape a loop
            break;
        }
        // prints 0 - 41
        println!("{:?}", x);
        // increment by 1. can still do 'x++' ... same as x=x+1
        x += 1;
    }
     // prints 42
    println!("{}", x);
    }

    //////////////////////////////////////////////
    //================== while loop

    // lets you add a condition to a loop.
    // If the condition becomes false, the loop will exit.
    fn whileLoop(){
        // initialize the variable x to the value of 0
        let mut x = 0;
        // when x becomes 5, exit the loop
        while x != 5 {
            // prints 0 - 4
            println!("{:?}", x);
            // keeps incrementing x by 1
            x += 1;
        }
        // prints 5
        println!("x is {}", x);
    }

    ///////////////////////////////////////////////
    //================= for loop
    /*
     * The .. operator
     creates an iterator that generates numbers from a start number up to but not including an end number.
     i.e;
     0..5 gives us 0,1,2,3,4
     * The ..= operator
     creates an iterator that generates numbers from a start number up to and including an end number.
        i.e;
     0..=5 gives us 0,1,2,3,4,5
     */

    fn forLoop(){
        // iterates over 0 - 4...
        for x in 0..5 {
            // ... and print them out
            println!("{}", x);
        }
        // iterates over 0 - 5...
        for x in 0..=5 {
            // ... and print them out
            println!("{}", x);
        }
    }

    ////////////////////////////////////////////////////
    // loop can break to return a value.

    fn returnLoop(){
        // initialize the variable 'x' to the value '0'
        let mut x = 0;
        // initialize the variable 'v' to a loop
        let v = loop {
        // keep adding 1, can still use x++
            x += 1;
            // if x gets to 13...
            if x == 13 {
                // ... break the loop and print
                break "found the 13";
            }
        };
        // prints 'found the 13'
        println!("from loop: {}", v);
    }
}
