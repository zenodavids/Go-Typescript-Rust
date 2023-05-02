# Enums

## Rust

> **Do you know that** you can **add types** to enums in rust?

```rs
enum RSEnum{
    Boo(i32),
    Foo(fn() -> i32), // function of type i32
    Bar(String), // string
    Baz(Vec<String>), // like an array of strings
}

// function to call through the function in the enum
fn  zooz() -> i32 {
    return 5
}

fn main(){
    // initialize the value '5' to the Boo enum child and initialize to the foo variable
    let foo = RSEnum::Boo(5);

    // pass in the function in the enum
    let foo = RSEnum::Foo(zooz);

    // let baz = RSEnum::Baz(["foo", "bar"]);

    /*create a variable on the ""if statement*/
    //if foo is a sub-type of foo, pluck out the value
    if let RSEnum::Foo(value) = foo {
        // some code...
    }

}

```

> **We can also do generics on enums :**

```rs
enum Foot<T> {
  Bar(T)
}

```

### another Enum example in **_rust_**

```rs
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let my_color = Color::Red; // Declare a variable `my_color` and set it to the `Red` variant of the `Color` enum.

    match my_color { // Use a `match` statement to check the value of `my_color` and execute code based on its variant.
        Color::Red => println!("The color is red!"),
        Color::Green => println!("The color is green!"),
        Color::Blue => println!("The color is blue!"),
    }
}

```

> In Rust, enums are a way to define a type by enumerating its possible variants. In this example, we define an enum called Color with three possible variants: Red, Green, and Blue. We then declare a variable my_color and set it to the Red variant of the Color enum.

> We use a match statement to check the value of my_color and execute code based on its variant. In this case, we print a message to the console indicating the color that was selected.

## GO

> **iota** auto increments values

```go
package main

import "fmt"

type Color int


const (
    Red Color = iota // Declare a constant `Red` of type `Color` and set it to `0`.
    Green            // Declare a constant `Green` of type `Color` and set it to `1`.
    Blue             // Declare a constant `Blue` of type `Color` and set it to `2`.
)

func main() {
    myColor := Red // Declare a variable `myColor` and set it to the `Red` constant.

    switch myColor { // Use a `switch` statement to check the value of `myColor` and execute code based on its value.
    case Red:
        fmt.Println("The color is red!")
    case Green:
        fmt.Println("The color is green!")
    case Blue:
        fmt.Println("The color is blue!")
    }
}

```

> In Go, enums are not explicitly defined, but can be simulated using const declarations. In this example, we declare a constant Color of type int and define three constants: Red, Green, and Blue.

> We then declare a variable myColor and set it to the Red constant. We use a switch statement to check the value of myColor and execute code based on its value. In this case, we print a message to the console indicating the color that was selected.

## Typescript

```ts
enum Color {
  Red,
  Green,
  Blue,
}

function printColor(color: Color): void {
  // Declare a function `printColor` that takes a `Color` parameter and returns `void`.
  switch (
    color // Use a `switch` statement to check the value of `color` and execute code based on its variant.
  ) {
    case Color.Red:
      console.log('The color is red!')
      break
    case Color.Green:
      console.log('The color is green!')
      break
    case Color.Blue:
      console.log('The color is blue!')
      break
    default:
      console.log('Unknown color!')
      break
  }
}

let myColor: Color = Color.Red // Declare a variable `myColor` and set it to the `Red` variant of the `Color` enum.
printColor(myColor) // Call the `printColor` function with `myColor` as an argument.
```

> In TypeScript, enums are similar to Rust enums, where we define a type by enumerating its possible variants. In this example, we define an enum called Color with three possible variants: Red, Green, and Blue.

> We also declare a function printColor that takes a Color parameter and returns void. We use a switch statement to check the value of the color parameter and execute code based on its variant. In this case, we print a message to the console indicating the color that was selected.

> We then declare a variable myColor and set it to the Red variant of the Color enum. Finally, we call the printColor function with myColor as an argument to print the message "The color is red!" to the console.
