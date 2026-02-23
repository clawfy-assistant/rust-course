# Solutions - Module 05: Error Handling

Complete solutions for the `?` operator and error propagation.

## Exercise 1: Read File Contents

Read an entire file into a String using the `?` operator.

```rust
use std::fs::File;
use std::io::{self, Read};

pub fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

**Key Points:**
- `?` automatically propagates errors (returns `Err` if operation fails)
- `File::open()` can fail (file not found, permissions)
- `read_to_string()` can fail (IO error, not valid UTF-8)
- No explicit `match` needed - `?` handles it
- Function return type must be `Result` compatible

**Without `?` operator:**
```rust
pub fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    // ... more verbose!
}
```

## Exercise 2: Config from Arguments

Parse command line arguments with custom error type.

```rust
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename })
    }
}
```

**Key Points:**
- Custom error type: `&'static str` for simple cases
- Early return on validation failure
- `args[0]` is the program name, so need 3 elements total
- Clone strings to own them (args are borrowed)

**Usage:**
```rust
let args: Vec<String> = env::args().collect();
let config = Config::new(&args)?;  // Propagate error with ?
```

## Exercise 3: Search Function

Find lines containing a query string.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

**Key Points:**
- Lifetime `'a` ties returned references to `contents` input
- `lines()` iterator over string lines
- `filter()` keeps only matching lines
- Zero-copy: returns references to original data

**With case-insensitive option:**
```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

## Error Handling Patterns

### Pattern 1: Custom Error Types

```rust
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Parse(String),
    NotFound(String),
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}
```

### Pattern 2: Result Combinators

```rust
// map: transform success value
let num = input.parse::<i32>().map(|n| n * 2)?;

// map_err: transform error
let num = input.parse::<i32>()
    .map_err(|e| format!("Invalid number: {}", e))?;

// unwrap_or: provide default
let num = input.parse::<i32>().unwrap_or(0);

// and_then: chain operations
result.and_then(|v| validate(v))?;
```

### Pattern 3: Multiple Operations

```rust
fn process_files(paths: &[&str]) -> Result<Vec<String>, io::Error> {
    let mut results = Vec::new();
    for path in paths {
        results.push(read_file_contents(path)?);
    }
    Ok(results)
}
```

## Summary Table

| Approach | Use When | Example |
|----------|----------|---------|
| `?` operator | Quick error propagation | `file.read_to_string(&mut s)?` |
| `match` | Custom error handling | Complex recovery logic |
| `unwrap()` | Prototype only (panics on error) | Quick tests |
| `expect()` | Unrecoverable errors | `expect("config file missing")` |
| `unwrap_or()` | Provide default value | `parse().unwrap_or(0)` |

## Tips

1. **Use `?` liberally** - Clean, readable error propagation
2. **Define custom errors** for libraries and complex apps
3. **`anyhow` crate** for simple error handling in applications
4. **`thiserror` crate** for defining custom error types
5. **Error messages matter** - Make them actionable
6. **Don't `unwrap()` in production** - Handle errors gracefully

## Mini grep Implementation

```rust
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_file_contents(&config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}
```
