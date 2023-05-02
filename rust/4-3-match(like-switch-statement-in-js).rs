/*
 * MATCH (works like SWITCH STATEMENT in JAVASCRIPT)
 matches all possible conditions of a value
 and executing a code path if the match is true.
*/
fn main() {
    let x = 42;

    match x {
        // if 0 matches...
        0 => println!("found zero"),
        // we can match against multiple values
        // if 1 or 2 matches...
        1 | 2 => println!("found 1 or 2!"),
 

        // we can match against ranges
        // if any number between 3 to 9 matches...
        3..=9 => 
            // print "found a number 3 to 9 inclusively"
            println!("found a number 3 to 9 inclusively"),
        // we can bind the matched number to a variable using '@'
        // if any number between 10 to 100 matches
        matched_num @ 10..=100 => println!("found number {} between 10 to 100!", matched_num),

        // this is the default match that must exist if not all cases are handled
        _ => println!("found something else!");
        
    }
}

// prefer match over if..else if working with a single variable
