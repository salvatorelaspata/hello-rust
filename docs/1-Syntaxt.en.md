# Syntax explained

## use std::io

```rust
use std::io; // It is a module
```

With `use` a module is imported. In this case the **module** `io` of the crate `std`.

## prelude

The compiler implicitly makes the most common types and functions available to you.

Normal practice, if necessary, is to add **prelude** to a module to make the most commonly used types / functions available to the programmer.

## fn main()

It is the main function of the program.

```rust
fn main() {
    // ...
}
```

> Can be passed arguments to the `main()` function.
> `std::env::args()` returns an iterator over the arguments passed to the program.

## let & mut

`let` is a statement that defines a **variable**.`

`mut` indicates that a variable is **mutable**.

In Rust there is no explicit statement to instantiate an object, unlike other languages that use new. Conventions are therefore used.

`String::new()` it is a function that returns a new instance of `String` with a variable size.

The syntax `::` indicates that the **new** function allocates a new instance of `String`.

The compiler **deduces** the type of the variable `guess` based on the value returned by the function `String::new()`. However, it is possible to specify the type of the variable.

```rust

let secret_number = rand::thread_rng().gen_range(1..101);

let mut guess: String = String::new();
```

> In Rust it is perhaps better not to specify the type of the variable, letting the compiler deduce it.

## io::stdin().read_line(&mut guess).expect("Failed to read line");

`io::stdin()` return an instance of `std::io::Stdin`.

`read_line(&mut guess)` invokes the method of `Stdin` that reads the user's input and stores it in `guess`.

`&` indicates that a **reference** of `guess` is passed to `read_line()`.

`expect("Failed to read line")` handles, badly, the exception in case of problems in the stdin interaction.

> In Rust there are no exceptions ... To be deepened.

## &mut guess

`&mut guess` is a **mutable reference** to `guess`.
