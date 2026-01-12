# 🦀 AI-Rust

### My Rust learning journey toward becoming a Rust Infra Engineer

#### Including my bloopers and baby steps 😂😭

---

## Preliminary Notes

Systems-level work that deals with low-level details like **memory management**, **data representation**, and **concurrency** — basically, "lower-level control."

- High level ergonomics and low level control are usually at odds, but rust challenges that.

- Rust can be used for low-level programming like building web servers, CLIs, and many other kinds of software.
- Safety, productivity, speed, and ergonomics are **not** trade-offs with Rust — it gives a balance of all.
- **Linker** — Rust uses this to join compiled outputs into one file.
- **rustup doc** — opens the local Rust documentation on your machine.
- Anytime a type or function comes from the standard library and you're unsure what it does, check the **API docs**.

---

## Core Concepts

- The `main` function is always the first code that runs in every executable Rust program.
- **rustc** is Rust's compiler. It reads your code, checks for errors, and compiles it into a binary executable your computer can run. In simple terms: _"Turn this Rust source code into a machine program."_
- When you use `!` like in `println!("Hello, world")` you are calling a **macro** as opposed to a function. Macros and functions follow different conventions.
- **Rust is an ahead-of-time compiled language**, meaning you can compile a program and give the executable to someone else, and they can run it without even having Rust installed.

---

## Example: What Rust Does When You Run `rustc main.rs`

### Code:

```rust
fn main() {
    println!("Hello, world");
}
```

### Compilation Process:

1. Reads your source code
2. Compiles it into a binary executable (machine instructions the computer understands)
3. Outputs a binary file in the same folder
   - **Windows:** `main.exe`
   - **macOS/Linux:** `main`

This new file is no longer Rust — it's a standalone program you can run directly.

### Running the Program

`./` means **run the executable in the current directory**.

```bash
./main          # macOS/Linux
.\main.exe      # Windows
```

---

## Hello, Cargo

### What is Cargo?

Cargo is Rust's **build system and package manager** (manages packages/dependencies - think NPM for JavaScript).

- **Cargo.toml** - TOML stands for "Tom's Obvious Minimal Language" (configuration file)
- **Crate** - a package of Rust code
- **Cargo.lock** - think of this as `package-lock.json`

### Cargo vs rustc

Unlike `rustc`, Cargo creates an executable file in a `target/debug` directory (because by default the build is a debug build).

Example: `./target/debug/hello-cargo` executable

what does `cargo build` do? it creates and executable in target/debug - because the default build is a debug build.
running `cargo build` for the first time creates a file at the top level `cargo.lock` - this tracks the exact versions of dependencies in your project think `package-lock.json`

`cargo run` compiles the code into the binary executable and runs it all at the same time. If no files changed after running `cargo build` or `cargo run` once, the next `cargo run` run will not contain compiling response.

`cargo check` is a handy and way faster way to confirm your code compiles while in development, without producing a binary executable.

### Building for Release

When your program is ready to be released to users, use:

```bash
cargo build --release
```

This compiles your program with optimizations. The binary executable will be created in `target/release` instead of `target/debug`. These optimizations make your Rust code run faster but increase compilation time.

---

## Variables and Mutability

Rust has a set of items defined in the standard library that it brings into the scope of every program. This set of items is called the **prelude**.

```rust
let apples = 5;      // immutable (variables are immutable by default in Rust)
let mut bananas = 5; // mutable (we added `mut` to make this variable mutable)
```

---

## Associated Functions

An **associated function** is a function that is implemented on a type.

Example: `String::new()`

- `String` is the type
- `new()` is the associated function
- It creates a new empty string

---

## Standard Input

`stdin()` is a function within the `io` module in the Rust standard library (`std::io`) that gives your program access to the standard input stream (what the user types).

**Important:** It does not directly give you the user's input. It gives you a **handle (object)** that you can then call methods like `read_line` on to access the user's input content.

```rust
io::stdin()  // Returns a handle to stdin
    .read_line(&mut guess)  // Method called on the handle
```

---

## Methods vs Functions

A **method** is still a function, it's just tied to an object.

---

## References and Mutability

```rust
read_line(&mut guess)
```

- `&` creates a **reference** to the `guess` variable without copying it into memory multiple times
- References are also **immutable by default** (like variables)
- We use `&mut` because we're giving permission to the `read_line` method to modify our `guess` variable
- Even though we used `mut` when declaring the variable, for security reasons, Rust requires you to explicitly give permission to modify your values through references

---

## Result Type

`read_line` returns a **Result** value.

- **Result** is an **enumeration (enum)** that can be in multiple possible states
- Each possible state is called a **variant**
- Result's variants are `Ok` and `Err`

### Using `.expect()`

```rust
.expect("Failed to read line")
```

This does two things:

1. If the result is `Err` - crash the program and print "Failed to read line"
2. If the result is `Ok` - return the value inside(in bytes ideally)

---

## How Input is Stored

When you type text, your computer stores it as **bytes**.

Examples:

- `hello` = 5 bytes (for simple English letters)
- `hi` + Enter = `h`, `i`, `\n` = Ok(3)

  - The `\n` (newline character) comes from pressing Enter

  ### Printing Values with println! Placeholders

  - think of {} as boxes where rust will put stuff you give it.
  - println!("x = {x}") → “Print the value of x” - Here Rust will replac {x} with whatever variable you have bound "x" to

- println!("y+2 {}, y+2) → “Print the value I’m giving you next after the comma”(the curly brackets is filled with whatever you pass after the comma)
- putting it together we have
  `let x = 5;`
  `let y = 10;`

`println!("x = {x} and y + 2 = {} ", y + 2)`

### Generating a Secret Number

- A crate is a collection of Rust source code files.
- the program we have been building is a binary crate which is an executable
- the rand crate is a library crate and cannot be executed on it's own
- rand 0.8.5 does not directly mean use version 0.8.5 but it's shorthand for ^0.8.5 meaning that all versions before version 0.9.0 can be used as version 0.9.0 may break our code. 0 is major version, 8 is minor version, 5 is patch.(semantic versioning)

- https://crates.io/ - open source rust projects library

### Generating a Random Number

- use rand::Rng; - this gives us access to random number functions from the rand crate/library
- you must write rand::Rng because the gen_range() function belongs to the Rng trait. to use it's functions, you have to import Rng
- rand::thread_rng() gets a random number generator for this program(gives us the particular random number generator we are making use of)
- .gen_range(1..=100) gives the random number a range from 1 to 100
- Another neat feature of Cargo is that running the `cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser.
