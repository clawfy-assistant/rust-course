# Solutions - Module 07: Lifetimes

Complete solutions for lifetime annotations.

## Exercise 1: Longest String

Return the longest of two string slices with shared lifetime.

```rust
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}
```

**Key Points:**
- `'a` is a lifetime parameter
- Both inputs and output have the same lifetime `'a`
- Returned reference is valid as long as **both** inputs are valid
- The compiler ensures no dangling references

**Visual Explanation:**
```
x: &'a str ──┐
             ├──→ returned &'a str valid for intersection of both lifetimes
y: &'a str ──┘
```

**Usage:**
```rust
let s1 = String::from("long string");
let s2 = "short";
let result = longest(s1.as_str(), s2);  // result lives as long as s1
```

## Exercise 2: Struct with References

A struct that holds a reference requires a lifetime annotation.

```rust
#[derive(Debug)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        Excerpt { part }
    }
    
    pub fn get_part(&self) -> &str {
        self.part
    }
}
```

**Key Points:**
- Struct holds a reference → needs lifetime parameter
- `impl<'a>` declares lifetime for implementation block
- `get_part()` elides lifetime (elision rules infer it)

**Lifetime Elision Rules (the compiler infers these):**
1. Each input reference gets its own lifetime
2. If exactly one input lifetime, it's assigned to all outputs
3. If `&self` or `&mut self`, its lifetime goes to all outputs

## Exercise 3: First Word

Extract the first word from a string slice.

```rust
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
```

**Key Points:**
- No explicit lifetime needed (elision rule #1 applies)
- Input `&str` and output `&str` get same lifetime automatically
- Returns slice up to first space, or entire string if no space

**Explicit version (what elision does):**
```rust
pub fn first_word<'a>(s: &'a str) -> &'a str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
```

## Lifetime Rules and Patterns

### The Three Elision Rules

1. **Each reference gets a lifetime**: `fn foo(x: &T)` → `fn foo<'a>(x: &'a T)`
2. **One input = output lifetime**: `fn foo(x: &T) -> &U` → `fn foo<'a>(x: &'a T) -> &'a U`
3. **Self gets priority**: `fn foo(&self, x: &T) -> &U` → `fn foo<'a, 'b>(&'a self, x: &'b T) -> &'a U`

### Multiple Lifetimes

```rust
// x and y have different lifetimes
fn longest_with_announcement<'a, 'b, T>(
    x: &'a str,
    y: &'b str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    x  // Only returns x, so only needs 'a
}
```

### Static Lifetime

```rust
// 'static means "lives for entire program duration"
let s: &'static str = "I'm a string literal, stored in binary";

// Use sparingly! Most 'static annotations are wrong
// Usually indicates you want owned data instead
```

## Common Lifetime Patterns

### Pattern 1: Borrow Checker Friendly

```rust
// GOOD: Clear ownership
pub fn analyze(data: &str) -> AnalysisResult {
    // Process data, return owned result
    AnalysisResult::new(data.len())
}

// AVOID: Fighting the borrow checker
pub fn analyze<'a>(data: &'a str) -> &'a AnalysisResult {
    // Trying to return reference to local data - WON'T COMPILE!
}
```

### Pattern 2: Struct with Lifetime

```rust
pub struct Parser<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser { source, position: 0 }
    }
    
    pub fn peek(&self) -> Option<&'a str> {
        self.source.get(self.position..self.position + 1)
    }
}
```

### Pattern 3: Lifetime Bounds

```rust
pub struct RefHolder<'a, T: 'a> {
    reference: &'a T,
}

// T: 'a means "T lives at least as long as 'a"
```

## Tips

1. **Start without lifetimes** - Let elision rules handle simple cases
2. **Add lifetimes when compiler asks** - Follow error messages
3. **Prefer owned types** - `String` instead of `&str` when possible
4. **Clone if needed** - Sometimes cloning is cleaner than complex lifetimes
5. **Return owned data** - Easier than managing reference lifetimes
6. **Use `Cow<'a, str>`** for "maybe borrowed, maybe owned" scenarios

## Troubleshooting

```rust
// ERROR: Returns reference to local variable
pub fn bad() -> &str {
    let s = String::from("hello");
    &s  // s dropped here, would be dangling!
}

// FIX 1: Return owned value
pub fn good1() -> String {
    String::from("hello")
}

// FIX 2: Return static string
pub fn good2() -> &'static str {
    "hello"
}

// FIX 3: Accept reference and return it
pub fn good3<'a>(input: &'a str) -> &'a str {
    input
}
```
