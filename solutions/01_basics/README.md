# Solutions - Module 01: Basics

Complete solutions for all exercises in 01_basics.

## Exercise 1: Variables and Mutability

```rust
/// Make x mutable and add 10
pub fn make_mutable_and_add_ten() -> i32 {
    let mut x = 5;  // Declare x as mutable
    x += 10;        // Add 10 to x
    x               // Return final value (15)
}

/// Swap values a and b using tuple destructuring
pub fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)  // Tuple destructuring - order matters!
}
```

## Exercise 2: Data Types

```rust
use std::f64::consts::PI;

/// Calculate circle area (πr²)
pub fn circle_area() -> f64 {
    let r = 2.5;
    PI * r * r  // π × r² = 3.14159... × 6.25 ≈ 19.6349
}

/// Extract first char and number from tuple
pub fn get_first_char_and_number(tup: (char, i32, bool)) -> (char, i32) {
    (tup.0, tup.1)  // Access tuple elements by index
}

/// Sum array elements
pub fn sum_array() -> i32 {
    let arr = [10, 20, 30, 40, 50];
    arr.iter().sum()  // Use iterator sum method
    // Or: arr[0] + arr[1] + arr[2] + arr[3] + arr[4]
}
```

## Exercise 3: Functions

```rust
/// Return greater of two values
pub fn max_of_two(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
    // Or: a.max(b)
}

/// Calculate factorial (n!)
pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)  // Recursive approach
    }
}
// factorial(5) = 5 × 4 × 3 × 2 × 1 = 120

/// Check if number is prime
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;  // 0 and 1 are not prime
    }
    if n <= 3 {
        return true;   // 2 and 3 are prime
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;  // Eliminate even numbers and multiples of 3
    }
    // Check remaining numbers up to √n
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;  // Skip by 6 to reduce checks
    }
    true
}
```

## Exercise 4: FizzBuzz

```rust
/// Classic FizzBuzz
pub fn fizzbuzz(n: i32) -> String {
    // Check divisibility by 15 first (both 3 and 5)
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()  // Convert number to string
    }
}
```

## Exercise 5: Loops

```rust
/// Sum 1 to n using for loop
pub fn sum_to_n(n: i32) -> i32 {
    (1..=n).sum()  // Range from 1 to n (inclusive), then sum
    // Or manual: 
    // let mut sum = 0;
    // for i in 1..=n { sum += i; }
    // sum
}
// Formula: n × (n + 1) / 2

/// Calculate nth Fibonacci number
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;  // F(n-2)
            let mut b = 1;  // F(n-1)
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b  // F(n)
        }
    }
}
// Sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...

/// Count non-space characters
pub fn count_chars(s: &str) -> usize {
    s.chars()
        .filter(|c| !c.is_whitespace())  // Keep only non-whitespace
        .count()
}
// "hello world" → filter out space → "helloworld" → 10 chars
```

## Key Concepts

1. **Mutability**: Use `let mut` for variables that change
2. **Tuples**: Access with `.0`, `.1`, etc.
3. **Ranges**: `1..=n` includes n, `1..n` excludes n
4. **Iterators**: `.iter().sum()`, `.filter()`, `.count()`
5. **Recursion**: Function calls itself (factorial)
6. **Pattern Matching**: `match` for multiple conditions