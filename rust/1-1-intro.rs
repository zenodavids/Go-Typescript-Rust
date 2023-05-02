// - Rust is a programming language developed by Mozilla Research.
// - Rust libraries as a “drop-in replacement” for C.
// rust has no garbage collector
// but achieves "memory safety" with a concept known OWNERSHIP & BORROWING

//////////////////////////
/// Casing Convention ///
////////////////////////
/* *
 * snake_case
Variables, Functions, Files.

 * SCREAMING_SNAKE_CASE
CONSTANTS, STATICS.

 * PascalCase
Types, Traits, Enums
*/

//////////////////
/// Variables ///
////////////////

let my_variable = 0;
const MY_CONSTANT: u8 = 0;
static MY_STATIC: u8 = 0;

// By default, all variables are immutable, allowing values to be used in the "stack memory".
// make a variable mutable by using the "mut" keyword and storing the values in the "Heap memory"

/////////////////////////////
/// running RUST locally ///
///////////////////////////

/* 
 for WINDOWS.
* in the root directory, open the terminal and first compile using;
rustc filename.rs
* then run;
.\main.exe
*/



