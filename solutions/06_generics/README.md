# Solutions - Module 06: Generics

Complete solutions for generic functions and structs.

## Exercise 1: Generic Swap

Swap two values of any type.

```rust
pub fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}
```

**Key Points:**
- `<T>` declares type parameter
- Both parameters must be same type `T`
- No runtime cost - monomorphization creates specialized code
- Can be used with any type: `swap(5, 10)`, `swap("a", "b")`

**With different types:**
```rust
pub fn swap_pair<T, U>(a: T, b: U) -> (U, T) {
    (b, a)
}
```

## Exercise 2: Largest Element

Find the largest element in a slice with trait bounds.

```rust
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}
```

**Key Points:**
- `PartialOrd` trait bound required for comparison (`>`, `<`)
- Returns `Option<&T>` to handle empty slices safely
- Returns reference to avoid cloning
- Works with any type implementing `PartialOrd`: numbers, strings, etc.

**Alternative syntax (where clause):**
```rust
pub fn largest<T>(list: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    // ... same body
}
```

## Exercise 3: Generic Point

A generic 2D point with specialized methods.

```rust
#[derive(Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

// Generic implementation for any T
impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Specialized implementation for f64 only
impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

**Key Points:**
- `impl<T>` implements methods for any type
- `impl Point<f64>` implements only for `Point<f64>`
- Specialization allows type-specific functionality
- `#[derive(PartialEq)]` works because `T: PartialEq` is implied

**Multiple type parameters:**
```rust
pub struct Point2<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Point2<T, U> {
    pub fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 { x: self.x, y: other.y }
    }
}
```

## Common Trait Bounds

| Trait | Provides | Example |
|-------|----------|---------|
| `PartialOrd` | Comparison (`<`, `>`, `<=`, `>=`) | Sorting, min/max |
| `Ord` | Total ordering | Binary trees, sets |
| `PartialEq` | Equality (`==`, `!=`) | Searching |
| `Eq` | Reflexive equality | HashMap keys |
| `Clone` | Deep copy | `.clone()` |
| `Copy` | Bitwise copy | Primitives, small structs |
| `Default` | Default value | `Default::default()` |
| `Display` | String formatting | `println!("{}")` |
| `Debug` | Debug formatting | `println!("{:?}")` |
| `Sized` | Known at compile time | Almost always implied |

## Advanced Patterns

### Multiple Trait Bounds

```rust
fn process<T: Clone + PartialOrd + Display>(item: &T) {
    // Can clone, compare, and display T
}

// Alternative syntax
fn process<T>(item: &T)
where
    T: Clone + PartialOrd + Display,
{
}
```

### Default Type Parameters

```rust
pub struct Counter<T = i32> {
    value: T,
}

// Counter::new() defaults to Counter<i32>
let c: Counter = Counter::new();  // T = i32
let c: Counter<f64> = Counter::new();  // T = f64
```

### Associated Types

```rust
pub trait Container {
    type Item;  // Associated type
    
    fn get(&self) -> Option<&Self::Item>;
}
```

## Generic Performance

Rust uses **monomorphization** - generics have **zero runtime cost**:

```rust
// Source code
let int = swap(5, 10);
let float = swap(1.5, 2.5);

// Compiler generates (roughly):
fn swap_i32(a: i32, b: i32) -> (i32, i32) { (b, a) }
fn swap_f64(a: f64, b: f64) -> (f64, f64) { (b, a) }
```

## Tips

1. **Start concrete, then generalize** - Write for one type first
2. **Use trait bounds liberally** - Document what operations are needed
3. **Prefer `where` clauses** for complex bounds - More readable
4. **Consider `impl Trait`** for return types: `fn foo() -> impl Iterator`
5. **Avoid over-genericizing** - Simple is better than complex
6. **Use turbofish when needed**: `parse::<i32>()` or `collect::<Vec<_>>()`

## Generic Data Structure Example

```rust
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
```
