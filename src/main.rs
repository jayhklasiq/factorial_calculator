use std::collections::HashMap;

// Define a struct to encapsulate factorial computation and storage
struct FactorialCalculator {
    cache: HashMap<u32, u64>,
}

impl FactorialCalculator {
    // Constructor to create a new instance of FactorialCalculator
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    // Method to calculate factorial of a number
    fn calculate_factorial(&mut self, n: u32) -> u64 {
        match self.cache.get(&n) {
            Some(&result) => result,
            None => {
                let result = if n == 0 {
                    1
                } else {
                    (1..=n).fold(1, |acc, x| acc * x as u64)
                };
                self.cache.insert(n, result);
                result
            }
        }
    }
}

fn main() {
    let mut calculator = FactorialCalculator::new();
    
    // Example usage
    println!("Factorial of 5: {}", calculator.calculate_factorial(5));
    println!("Factorial of 3: {}", calculator.calculate_factorial(3));
    println!("Factorial of 8: {}", calculator.calculate_factorial(8));
}
