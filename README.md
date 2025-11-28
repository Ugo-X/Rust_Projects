# 🦀 AI-Rust

### My Rust learning journey toward becoming a Rust Infra Engineer

#### Including my bloopers and baby steps 😂😭

---

## Preliminary Notes

Systems-level work that deals with low-level details like **memory management**, **data representation**, and **concurrency** — basically, “lower-level control.”

* Rust can be used for low-level programming like building web servers, CLIs, and many other kinds of software.
* Safety, productivity, speed, and ergonomics are **not** trade-offs with Rust — it gives a balance of all.
* **Linker** — Rust uses this to join compiled outputs into one file.
* **rustup doc** — opens the local Rust documentation on your machine.
* Anytime a type or function comes from the standard library and you’re unsure what it does, check the **API docs**.

---

## Core Concepts

* The `main` function is always the first code that runs in every executable Rust program.
* **rustc** is Rust’s compiler. It:

  * reads your code,
  * checks for errors,
  * compiles it into a binary executable your computer can run.
  * In simple terms: *“Turn this Rust source code into a machine program.”*

---

## Example

### Code:

```rust
fn main() {
    println!("Hello, world");
}
```

### What Rust Does When You Run `rustc main.rs`:

1. Reads your source code
2. Compiles it into a binary executable (machine instructions the computer understands)
3. Outputs a binary file in the same folder

Platform differences:

* **Windows:** `main.exe`
* **macOS/Linux:** `main`

This new file is no longer Rust — it’s a standalone program you can run directly.

---

## Running the Program

### What does `./main` mean?

`./` means **run the executable in the current directory**.

So when you type:

```bash
./main on macos or .\main on windows
```

You are running the compiled binary executable.

- when you use ! like in the function body;
- println!("Hello, world")  you are calling a macro as opposed to a function and macros and functions mostly follow different conventions
- `Rust is an ahead of time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it without even having rust installed.`

## Hello, Cargo

### What does `Cargo` mean?
- Cargo is Rust's build system and package manager(manages packages/dependencies- think NPM)    

- cargo.toml - tom's obvious minimum language
- packages of code - crate
- unlike rustc, Cargo creates an executable file(binary executable in a directory "debug" cause by default the build is a debug build) eg - hello-cargo executable
- think of cargo.lock as package-lock.json

### Building for Release
when your program is ready to be released to users, you can use cargo build --release to compile your program with optimizations. the binary executable will be created in `target/release` as opposed to `/target/debug`. These optimisations make your rust code run faster but increases the compilation time.