// Rust Control Flow
fn main() {
    // Rust if...else
    let number = 10;
    if number % 2 == 0 {
        println!("Number is even");
    } else {
        println!("Number is odd");
    }

    // Rust loop
    loop {
        println!("This is an infinite loop");
        break; // Use break to exit the loop
    }

    // Rust while Loop
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // Rust for Loop
    let numbers = [1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // Rust break and continue
    for num in 1..=10 {
        if num % 3 == 0 {
            continue; // Skip the rest of the loop body for multiples of 3
        }
        println!("Number: {}", num);
    }

    // Rust Data Types
    // Rust Array
    let array: [i32; 3] = [1, 2, 3];

    // Rust Slice
    let slice = &array[1..2];

    // Rust Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    // Rust Struct
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 10, y: 5 };

    // Rust Functions
    // Rust Function
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Rust Variable Scope
    {
        let x = 5; // x is valid only within this block
        println!("x: {}", x);
    }

    // Rust Closure
    let add_closure = |a, b| a + b;
    println!("Closure Result: {}", add_closure(2, 3));

    // Rust Standard Library
    // Rust Stack and Heap
    let stack_var = 42; // stored on the stack
    let heap_var = Box::new(42); // stored on the heap

    // Rust Vector
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Rust String
    let mut s = String::from("Hello, ");
    s.push_str("world!");

    // Rust HashMap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    // Rust HashSet
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);

    // Rust Iterator
    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // Rust Error and Memory
    // Rust Error Handling
    fn read_username_from_file() -> Result<String, std::io::Error> {
        // implementation of reading from a file
        Ok(String::from("username"))
    }

    // Rust Unwrap and Expect
    let result = read_username_from_file().unwrap_or_else(|err| {
        panic!("Failed to read username: {}", err);
    });

    // Rust Ownership
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                  // but i32 is Copy, so it’s okay to still
                  // use x afterward

    // Rust References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);

    // Rust Modules and Packages
    // Rust Modules
    mod mymodule {
        pub fn myfunction() {
            println!("This is my function inside the module");
        }
    }

    // Rust Crate and Package
    // In the Cargo.toml file:
    // [package]
    // name = "myproject"
    // version = "0.1.0"

    // Rust Cargo Package Manager
    // Commands: cargo new myproject, cargo build, cargo run, cargo test, cargo publish

    // Rust Additional Topics
    // (Additional topics can be implemented here)
}

fn takes_ownership(some_string: String) {
    // do something with some_string
}

fn makes_copy(some_integer: i32) {
    // do something with some_integer
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
