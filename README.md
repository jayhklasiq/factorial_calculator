# Overview

As a software engineer striving to learn the basics Rust programming, I developed this simple factorial calculator to showcase the language's syntax, ownership, and caching mechanism. The software aims to demonstrate Rust's efficiency and safety, which are fundamental aspects of the language.

## Software Demo Video

[Watch the Software Demo Video](http://youtube.link.goes.here)

The main components of this code is below.

_std::collections::HashMap_: This is a generic hash map provided by Rust's standard library. It is used as a cache to store previously computed factorials.

_FactorialCalculator struct_: Defines a struct that encapsulates factorial computation and storage. It has a single field, cache, which is a HashMap that stores previously computed factorials.

_calculate_factorial method_: This method calculates the factorial of a given number n. If the factorial of n is already in the cache, it returns the cached value. Otherwise, it computes the factorial and stores it in the cache before returning the result.

_new constructor_: This constructor initializes a new FactorialCalculator instance with an empty cache.

_main function_: This is the entry point of the program. It creates a new FactorialCalculator, calculates the factorials of 5, 3, and 8, and prints the results.

# Here's a brief example of how this code works:

1. Create a new FactorialCalculator instance called calculator.
2. Request the factorial of 5. The calculator computes it as 5! = 5 _ 4 _ 3 _ 2 _ 1 = 120 and stores the result in the cache.
3. Request the factorial of 3. The calculator checks the cache and finds that 3! has already been computed as 3 _ 2 _ 1 = 6. It returns the cached value.
4. Request the factorial of 8. The calculator computes it as 8! = 8 _ 7 _ 6 _ 5 _ 4 _ 3 _ 2 \* 1 = 40320 and stores the result in the cache.
5. The main advantage of this implementation is that it avoids redundant computations. If the factorial of a given number has already been calculated, the calculator simply returns the cached value, resulting in faster computations.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

- **Programming Language:** Rust
- **Libraries:** std::collections::HashMap
- **Text Editor/IDE:** Visual Studio Code
- **Rust Compiler:** Rustc
- **Package Manager:** Cargo

# Useful Websites

- [Rust Programming Language Official Website](https://www.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Rust Essential Training](https://www.linkedin.com/learning/rust-essential-training/solution-max-min-mean?u=2153100)

# Future Work

- Add unit tests for the `calculate_factorial` method to ensure correctness.
- Implement a command-line interface to accept user input and display the factorial result.
- Explore other caching techniques and data structures for improved performance.
