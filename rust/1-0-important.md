### start a new rust project:

```sh
cargo init project_name
```

> **Never use the _mut_ keyword unless you _really_ need to**

# The Three Simple rules

## Value

> only **one** value owner
> What the variable is initialized to or the value of any variable.

```rs
fn main(){
    let x = 5;
    let y = x; // same in javascript as "let b = a"
    println!("{}", x + y); // logs "5 + 5"

}
```

## Reference

> You can have has many references as you like with the constraints that there are **no Mutable References alive**

Can make reference to the value - this means **point to the location where it is stored in memory without actually copying it**. It's like telling someone where a toy is located without actually giving them the toy to play with. In programming, making a reference to a value allows you **to use that value without making a copy of it, which can save time and memory**.

```rs
fn main(){
    let x = 5;
    let y = &x; // same in javascript as "let b = a"
    println!("here is {} and {}", x, y);

}
```

## Mutable Reference

> You can only have one mutable reference and no reference

reference to a value that can also be mutated - a type of reference in Rust that allows you to **point to a value in memory and also modify that value**. It's like giving someone a toy to play with and then letting them change it however they want.

In Rust, you can create a mutable reference using the &mut syntax. This type of reference allows you to modify the value it points to, but there are some rules around how you can use it.

```rs
fn main(){
    let mut x = 5; // "mut" makes it changeable like "let" in javascript
    let y = &x; // borrow "x" as immutable
    let z = &mut x; // cannot borrow "x" as mutable because it's already borrowed as immutable
    println!("{}", x + y + z);

}
```

> **Difference between a reference and a mutable reference is;**
> A reference only allows you to **read** a data
> Mutable reference allows you to **mutate** the underlined item.
