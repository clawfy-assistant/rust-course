# Solutions - Module 03: Structs and Enums

Complete solutions for data structures.

## Exercise 1: Structs

```rust
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Constructor
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    /// Calculate area
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Check if square
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Check if can contain another rectangle
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
```

**Usage**:
```rust
let rect = Rectangle::new(10, 20);
println!("Area: {}", rect.area());  // 200
println!("Is square: {}", rect.is_square());  // false
```

## Exercise 2: Enums

### Direction Enum

```rust
#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Get opposite direction
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}
```

### Message Enum with Data

```rust
#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,                           // No data
    Move { x: i32, y: i32 },       // Named fields (like struct)
    Write(String),                  // Single value
    ChangeColor(u8, u8, u8),        // Multiple values (RGB)
}

impl Message {
    /// Process message and return description
    pub fn process(&self) -> String {
        match self {
            Message::Quit => "Quitting...".to_string(),
            Message::Move { x, y } => {
                format!("Moving to {}, {}", x, y)
            }
            Message::Write(text) => {
                format!("Writing: {}", text)
            }
            Message::ChangeColor(r, g, b) => {
                format!("Changing color to {}, {}, {}", r, g, b)
            }
        }
    }
}
```

**Usage**:
```rust
let msg = Message::Move { x: 10, y: 20 };
println!("{}", msg.process());  // "Moving to 10, 20"
```

## Exercise 3: Option

```rust
/// Find index of target in vector
pub fn find_index(vec: &[i32], target: i32) -> Option<usize> {
    for (i, &v) in vec.iter().enumerate() {
        if v == target {
            return Some(i);  // Found!
        }
    }
    None  // Not found
}

/// Get first element (generic)
pub fn first_element<T>(vec: &[T]) -> Option<&T> {
    vec.first()  // Built-in method
    // Or: if vec.is_empty() { None } else { Some(&vec[0]) }
}

/// Add two Option values
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,  // If either is None, return None
    }
}
```

**Key Points**:
- `Option<T>` = `Some(T)` or `None`
- Use `match` or `if let` to extract values
- `?` operator can propagate Option

## Exercise 4: Result

```rust
#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    InvalidNumber,
}

/// Parse string to number with error handling
pub fn parse_number(s: &str) -> Result<i32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyString);
    }
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::InvalidNumber),
    }
}

/// Safe division
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Sum results, fail fast on error
pub fn sum_results(results: Vec<Result<i32, &str>>) -> Result<i32, &str> {
    let mut sum = 0;
    for result in results {
        match result {
            Ok(n) => sum += n,
            Err(e) => return Err(e),  // Early return on error
        }
    }
    Ok(sum)
}
```

**Key Points**:
- `Result<T, E>` = `Ok(T)` (success) or `Err(E)` (failure)
- Define custom error types as enums
- `?` operator propagates errors automatically

## Exercise 5: Pattern Matching

```rust
#[derive(Debug, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),  // State that produced it
}

/// Get coin value in cents
pub fn coin_value(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,  // Ignore state with _
    }
}

/// Count quarters and sum total
pub fn count_quarters(coins: &[Coin]) -> (usize, u32) {
    let mut count = 0;
    let mut total = 0u32;
    
    for coin in coins {
        match coin {
            Coin::Quarter(state) => {
                println!("Quarter from {}!", state);
                count += 1;
                total += 25;
            }
            other => {
                total += coin_value(other) as u32;
            }
        }
    }
    (count, total)
}
```

**Pattern Matching Features**:
- Match on enum variants
- Extract data: `Coin::Quarter(state)` binds state
- Ignore with `_`
- Match guards: `Coin::Quarter(state) if state == "Alaska"`

## Summary Table

| Type | Use When | Example |
|------|----------|---------|
| **Struct** | Group related data | `Rectangle { width, height }` |
| **Enum** | One of many options | `Direction::North` |
| **Option** | Value may be absent | `Some(5)` or `None` |
| **Result** | Operation may fail | `Ok(value)` or `Err(error)` |

## Tips

1. Use `#[derive(Debug, PartialEq)]` for printing and comparing
2. `impl` blocks add methods to types
3. Pattern matching with `match` is exhaustive (must handle all cases)
4. `if let` for single pattern matching
5. `?` operator simplifies error propagation