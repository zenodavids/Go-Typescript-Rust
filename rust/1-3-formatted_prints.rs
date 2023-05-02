// Printing is handled by a series of macros defined in std::fmt some of which include:

// more docs => https://doc.rust-lang.org/std/fmt/
fn main() {
/*
format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as print! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint! but a newline is appended.
All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.
*/

    // In general, the `{}` will be automatically replaced with any arguments.
    // These will be stringified.
    println!("{} days", 31);

    // Specifying an integer inside `{}`
    // determines which additional argument will be replaced.
    // Arguments start at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{argument}", argument = "test");   // => "test"
    println!("{name} {}", 1, name = 2);          // => "2 1"
    println!("{a} {c} {b}", a="a", b='b', c=3);  // => "a 3 b"
    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character after a
    // `:`.
    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // You can right-align text with a specified width. This will output
    // "    1". 4 white spaces and a "1", for a total width of 5.
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes.
    println!("{number:0>5}", number=1);         // This will output "00001"

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);         // This will output "00001"
    println!("{:04}", 42);             // => "0042" with leading zeros
    println!("{1} {} {0} {}", 1, 2);          // => "2 1 1 2"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // working with pi, lets round up to 3 decimal places
    let pi = 3.141592;
    let decimal_places = 3;
    println!("{:.1$}", pi, decimal_place); // prints '3.142'.
    
}

