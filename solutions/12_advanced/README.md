# Solutions - Module 12: Advanced Topics

Complete solutions for unsafe code, macros, and advanced types.

## Exercise 1: Unsafe Functions

Mark functions that require unsafe to call.

```rust
/// Unsafe: Caller must ensure safety conditions
pub unsafe fn dangerous() -> i32 {
    42
}
```

**Key Points:**
- `unsafe fn` declares a function with safety requirements
- Caller must use `unsafe` block to call
- Function body is still checked by compiler
- Safety contract is documented, not enforced

## Exercise 2: Unsafe Blocks

Contain unsafe operations within safe APIs.

```rust
pub fn use_unsafe() -> i32 {
    unsafe {
        dangerous()
    }
}
```

**Key Points:**
- `unsafe {}` block contains unsafe operations
- Code inside still has some checks (not "anything goes")
- Wrap unsafe in safe abstractions when possible
- Document invariants that must be upheld

**Common unsafe operations:**
```rust
unsafe {
    // Dereference raw pointer
    let x = *raw_ptr;
    
    // Call unsafe function
    dangerous();
    
    // Access static mutable variable
    COUNTER += 1;
    
    // Implement unsafe trait
    impl UnsafeTrait for MyType {}
    
    // Access union fields
    let x = my_union.value;
}
```

## Exercise 3: Declarative Macros

Define macros with `macro_rules!`.

```rust
#[macro_export]
macro_rules! say_hello {
    // No arguments
    () => {
        println!("Hello!");
    };
    
    // One expression argument
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}
```

**Key Points:**
- `#[macro_export]` makes macro public
- Pattern matching on syntax
- `$name:expr` captures expression
- Multiple arms like `match`
- Must end with semicolon (inside macro)

**Usage:**
```rust
say_hello!();           // Hello!
say_hello!("world");    // Hello, world!
say_hello!(42);         // Hello, 42!
```

**More complex macro:**
```rust
#[macro_export]
macro_rules! vec_of_strings {
    // Match any number of expressions
    ($($item:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($item.to_string());
            )*
            temp_vec
        }
    };
}

// Usage:
// vec_of_strings!("a", "b", "c") → Vec<String>
```

## Exercise 4: Type Aliases

Create alternative names for existing types.

```rust
/// Type alias for clarity
type Kilometers = i32;

/// Type alias for complex type
type Thunk = Box<dyn Fn() + Send + 'static>;
```

**Key Points:**
- Type aliases don't create new types (just synonyms)
- `Kilometers` and `i32` are interchangeable
- Useful for long, complex types
- Can't implement traits on type aliases

**Function pointer alias:**
```rust
type Callback = fn(i32, i32) -> i32;

fn apply(x: i32, y: i32, f: Callback) -> i32 {
    f(x, y)
}
```

## Exercise 5: Diverging Functions

Functions that never return (bottom type `!`).

```rust
/// Never type (diverging function)
pub fn never_returns() -> ! {
    loop {
        println!("forever");
    }
}
```

**Key Points:**
- `!` is the "never" type (no value)
- Function loops forever, exits process, or panics
- Can coerce to any type (useful in `match`)

**Common diverging functions:**
```rust
fn exit(code: i32) -> ! { std::process::exit(code) }
fn panic(msg: &str) -> ! { /* ... */ }
fn infinite_loop() -> ! { loop {} }

// Useful in match arms
let x: i32 = match result {
    Ok(v) => v,
    Err(e) => panic!("Error: {}", e),  // ! can become i32
};
```

## Unsafe Deep Dive

### Raw Pointers

```rust
let mut num = 5;

// Create raw pointers
let r1 = &num as *const i32;    // Immutable raw pointer
let r2 = &mut num as *mut i32;  // Mutable raw pointer

unsafe {
    // Dereference raw pointers
    println!("r1 is: {}", *r1);
    *r2 = 10;
}
```

**Raw pointers vs references:**
- Can be null
- Don't enforce borrowing rules
- No automatic cleanup
- Can point to any memory location

### FFI (Foreign Function Interface)

```rust
// Link to C library
#[link(name = "mylib")]
extern "C" {
    fn c_function(x: i32) -> i32;
}

// Safe wrapper
pub fn safe_c_call(x: i32) -> i32 {
    unsafe { c_function(x) }
}
```

### Unsafe Traits

```rust
unsafe trait UnsafeTrait {
    // methods
}

unsafe impl UnsafeTrait for MyType {
    // Must uphold trait's safety invariants
}
```

## Procedural Macros

### Derive Macro

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

### Attribute-like Macro

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Transform annotated item
}

// Usage:
#[route("/")]
fn index() { }
```

## Advanced Type System

### PhantomData

```rust
use std::marker::PhantomData;

struct MyStruct<T> {
    data: u32,
    _phantom: PhantomData<T>,  // Pretend to hold T
}

// T affects type but not runtime representation
let x: MyStruct<String> = MyStruct { data: 42, _phantom: PhantomData };
let y: MyStruct<i32> = MyStruct { data: 42, _phantom: PhantomData };
// x and y have different types!
```

### Existential Types (impl Trait)

```rust
// Return concrete type as trait
pub fn make_iter() -> impl Iterator<Item = i32> {
    0..10
}

// Argument position (different meaning!)
pub fn use_iter(iter: impl Iterator<Item = i32>) {
    // ...
}
```

### Const Generics

```rust
struct Array<T, const N: usize> {
    data: [T; N],
}

// Different sizes = different types
let a: Array<i32, 5> = Array { data: [0; 5] };
let b: Array<i32, 10> = Array { data: [0; 10] };
```

## Tips

1. **Minimize unsafe** - Isolate and document thoroughly
2. **Safe abstractions** - Wrap unsafe in safe APIs
3. **Use `std::ptr` methods** - `read_volatile`, `write`, `drop_in_place`
4. **Miri tool** - Test unsafe code for UB detection
5. **Macros are code** - Keep them simple and readable
6. **Procedural macros** need separate crate
7. **Type aliases** for readability, newtypes for safety
8. **Zero-cost abstractions** - Compiler optimizes through indirection

## When to Use Unsafe

| Use Case | Safer Alternative |
|----------|-------------------|
| FFI (C interop) | `bindgen` for automatic bindings |
| Performance critical | `unsafe` after profiling |
| OS/embedded dev | `embedded-hal` abstractions |
| Custom data structures | `std::collections` first |
| Raw pointer arithmetic | Slices and iterators |

## Summary Table

| Feature | Use For | Example |
|---------|---------|---------|
| `unsafe fn` | Functions with safety requirements | FFI wrappers |
| `unsafe {}` | Raw pointers, FFI, static mut | Pointer deref |
| `macro_rules!` | Code generation, DSLs | `vec![]` |
| `type` alias | Complex type synonyms | Callback types |
| `!` (never) | Panic, exit, infinite loops | `panic!()` |
| `const` generics | Types parameterized by values | Fixed arrays |
| `impl Trait` | Hide implementation details | Iterator returns |
