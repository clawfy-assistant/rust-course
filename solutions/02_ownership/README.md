# Solutions - Module 02: Ownership and Borrowing

Complete solutions for ownership exercises.

## Exercise 1: Understanding Move

```rust
/// Demonstrate ownership move
pub fn ownership_demo() -> String {
    let s1 = String::from("Rust");  // s1 owns "Rust"
    let s2 = s1;                     // s1 MOVED to s2, s1 is now invalid
    s2  // Return s2 (s1 cannot be used here!)
}
```

**Key Point**: In Rust, `String` is moved, not copied. After `let s2 = s1;`, `s1` is invalid.

## Exercise 2: Borrowing

```rust
/// Get length without taking ownership
pub fn get_length(s: &String) -> usize {
    s.len()  // Borrow s immutably
}
// s is still valid after this call!

/// Append " World!" without ownership
pub fn append_world(s: &str) -> String {
    format!("{} World!", s)  // Create NEW string, don't modify s
}
```

**Key Points**:
- `&String` = borrow String immutably
- `&str` = string slice (works with String and string literals)
- Borrowed values are returned after use

## Exercise 3: Mutable Borrowing

```rust
/// Add "!" in-place
pub fn exclaim(s: &mut String) {
    s.push('!');  // Mutate the borrowed value
}
// Usage:
// let mut s = String::from("Hello");
// exclaim(&mut s);  // s is now "Hello!"

/// Swap two values
pub fn swap_values(a: &mut i32, b: &mut i32) {
    let temp = *a;  // Dereference to get value
    *a = *b;
    *b = temp;
}
// Usage:
// let mut x = 5;
// let mut y = 10;
// swap_values(&mut x, &mut y);  // x=10, y=5
```

**Key Points**:
- `&mut T` = mutable borrow (can modify)
- Only ONE mutable borrow allowed at a time
- Use `*` to dereference and get/set value

## Exercise 4: String Slices

```rust
/// Get first word (until first space)
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],  // Slice from start to space
        None => s,           // No space, return entire string
    }
}
// "hello world" → "hello"
// "rust" → "rust"

/// Get last word
pub fn last_word(s: &str) -> &str {
    match s.rfind(' ') {      // rfind = find from right
        Some(i) => &s[i+1..], // Slice from after space to end
        None => s,
    }
}
// "hello world" → "world"

/// Get substring with bounds checking
pub fn substring(s: &str, start: usize, end: usize) -> &str {
    // Validate bounds
    if start >= s.len() || end > s.len() || start >= end {
        return "";
    }
    &s[start..end]
}
// substring("hello", 1, 4) → "ell"
```

**Key Points**:
- String slices `&str` are references to part of a string
- `&s[start..end]` creates a slice
- Slices borrow from original string (must stay in scope)

## Exercise 5: Multiple Borrows

```rust
/// Calculate sum and average
pub fn sum_and_average(numbers: &[i32]) -> (i32, f64) {
    if numbers.is_empty() {
        return (0, 0.0);
    }
    let sum: i32 = numbers.iter().sum();
    let avg = sum as f64 / numbers.len() as f64;
    (sum, avg)
}
// Borrows slice immutably, can read all elements

/// Find min and max
pub fn find_min_max(numbers: &[i32]) -> Option<(i32, i32)> {
    if numbers.is_empty() {
        return None;
    }
    let mut min = numbers[0];
    let mut max = numbers[0];
    for &n in numbers.iter().skip(1) {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    Some((min, max))
}
```

**Key Points**:
- `&[T]` = borrowed slice (can read multiple elements)
- Can have multiple immutable borrows simultaneously
- `iter().skip(1)` skips first element

## Exercise 6: Ownership with Structs

```rust
pub struct Person {
    name: String,  // Person owns the name
    age: u32,
}

impl Person {
    /// Create new Person
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),  // Copy from &str to String
            age,
        }
    }

    /// Borrow name (don't move)
    pub fn get_name(&self) -> &str {
        &self.name  // Return reference, not ownership
    }

    /// Modify age (requires mutable borrow)
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }
}
```

**Usage**:
```rust
let mut person = Person::new("Alice", 25);
let name = person.get_name();  // Immutable borrow
println!("{}", name);          // "Alice"
person.have_birthday();        // Mutable borrow
// person.age is now 26
```

## Ownership Rules Summary

1. **One owner**: Each value has exactly one owner
2. **Move**: When you assign `String`, ownership moves
3. **Borrow**: Use `&` to borrow without taking ownership
4. **Mutable borrow**: Use `&mut` to modify borrowed value
5. **One mutable OR many immutable**: Can't have both at once
6. **Slices**: `&str` and `&[T]` borrow parts of collections