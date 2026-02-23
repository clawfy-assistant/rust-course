# Solutions - Module 08: Traits

Complete solutions for defining and implementing traits.

## Exercise 1: Summary Trait

Define a trait with required and default methods.

```rust
pub trait Summary {
    // Required method - must be implemented
    fn summarize(&self) -> String;
    
    // Default implementation - can be overridden
    fn summarize_author(&self) -> String {
        String::from("(read more...)")
    }
}
```

**Key Points:**
- Traits define shared behavior (interfaces)
- Methods without body are required
- Methods with body are defaults (can be overridden)
- Multiple traits can be implemented for one type

## Exercise 2: Implementing Summary

Implement `Summary` for different types.

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", 
            self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

**Key Points:**
- `impl TraitName for Type` syntax
- Must implement all required trait methods
- Can use default implementations or override them
- Same trait, different behavior per type

## Exercise 3: Trait as Parameter

Accept any type implementing a trait.

```rust
// Syntax sugar for trait bound
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Equivalent explicit syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

**Key Points:**
- `&impl Trait` is concise syntax
- `<T: Trait>` allows specifying `T` multiple times
- Both approaches are equivalent for single use

**Multiple trait bounds:**
```rust
pub fn notify(item: &(impl Summary + Display)) { }

pub fn notify<T: Summary + Display>(item: &T) { }
```

## Exercise 4: Trait Bounds

Use traits to constrain generic types.

```rust
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

**Key Points:**
- `PartialOrd` enables comparison operators
- Trait bounds limit what types can be used
- Compiler guarantees type supports required operations

## Exercise 5: Orphan Rule and Newtype

Implement trait for external types using newtype pattern.

```rust
pub trait Displayable {
    fn display(&self);
}

// Can't implement foreign trait on foreign type
// But CAN implement your trait on foreign type
impl Displayable for i32 {
    fn display(&self) {
        println!("The number is: {}", self);
    }
}
```

**Newtype Pattern** (when you need both foreign):
```rust
pub struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

## Advanced Trait Patterns

### Associated Types

```rust
pub trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation specifies the type
impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

### Default Generic Types

```rust
pub trait Add<Rhs = Self> {
    type Output;
    
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Can add Milliseconds to Seconds
impl Add<Seconds> for Milliseconds {
    type Output = Milliseconds;
    fn add(self, rhs: Seconds) -> Self::Output {
        Milliseconds(self.0 + rhs.0 * 1000)
    }
}
```

### Supertraits

```rust
// OutlinePrint requires Display
pub trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

## Trait Object vs Generics

```rust
// Static dispatch - monomorphization, faster
pub fn notify<T: Summary>(item: &T) { }

// Dynamic dispatch - vtable lookup, flexible
pub fn notify(item: &dyn Summary) { }

// Must be object-safe:
// - No generic methods
// - No Self: Sized bounds
```

## Common Standard Library Traits

| Trait | Purpose | Methods |
|-------|---------|---------|
| `Clone` | Deep copy | `.clone()` |
| `Copy` | Bitwise copy (implicit) | Marker trait |
| `Debug` | Debug formatting | `{:?}` |
| `Display` | User-facing format | `{}` |
| `Default` | Default value | `Default::default()` |
| `PartialEq` | Equality (`==`) | - |
| `PartialOrd` | Ordering | `<`, `>`, etc. |
| `Eq` | Reflexive equality | Marker trait |
| `Ord` | Total ordering | `cmp()` |
| `Hash` | Hashing | `hash()` |
| `Drop` | Cleanup | `drop()` |
| `Sized` | Known size at compile | Marker trait |
| `Send` | Safe to send between threads | Marker trait |
| `Sync` | Safe to share between threads | Marker trait |

## Derivable Traits

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}
```

## Tips

1. **Implement common traits** - `Debug`, `Clone`, `PartialEq` are expected
2. **Use `#[derive(...)]`** when possible - Less boilerplate
3. **Document trait contracts** - What must implementers guarantee?
4. **Keep traits focused** - One concept per trait
5. **Use associated types** for type families
6. **Blanket implementations** are powerful: `impl<T: Display> ToString for T`
