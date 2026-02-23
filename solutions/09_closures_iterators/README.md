# Solutions - Module 09: Closures and Iterators

Complete solutions for functional programming patterns.

## Exercise 1: Closure Factory

Create closures that capture their environment.

```rust
pub fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}
```

**Key Points:**
- `move` keyword transfers ownership of captured variables
- `impl Fn(i32) -> i32` hides the closure's concrete type
- Closures can capture variables from enclosing scope
- Each closure has a unique, unnameable type

**Usage:**
```rust
let triple = make_multiplier(3);
assert_eq!(triple(4), 12);  // 4 * 3 = 12
assert_eq!(triple(5), 15);  // 5 * 3 = 15
```

**Closure Traits:**
- `Fn`: Borrows values immutably (most common)
- `FnMut`: Borrows values mutably
- `FnOnce`: Takes ownership (can only call once)

## Exercise 2: Filter and Map

Process collections with iterator adapters.

```rust
pub fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&x| x > 0)    // Keep only positive numbers
        .map(|x| x * 2)          // Double each
        .collect()               // Collect into Vec
}
```

**Key Points:**
- `iter()` creates an iterator over references
- `filter()` keeps elements where predicate is true
- `map()` transforms each element
- `collect()` gathers results into a collection
- **Lazy evaluation**: Nothing happens until `collect()`

**Without double reference:**
```rust
.filter(|x| **x > 0)  // x is &&i32, need **
// Or use pattern matching:
.filter(|&&x| x > 0)  // Destructure in pattern
```

## Exercise 3: Sum of Evens

Filter and sum in one chain.

```rust
pub fn sum_of_evens(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // Keep even numbers
        .sum()                       // Sum them all
}
```

**Key Points:**
- `sum()` is a consuming adapter (returns value, not iterator)
- Type inference works for numeric types
- Empty iterator sums to 0

## Exercise 4: Uppercase Names Filter

Create an iterator that filters and returns owned values.

```rust
pub fn uppercase_names(names: &[String]) -> impl Iterator<Item = String> + '_ {
    names
        .iter()
        .filter(|name| name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
        .cloned()
}
```

**Key Points:**
- `impl Iterator<Item = String>` returns an iterator trait object
- `'_` lifetime ties returned iterator to input slice
- `cloned()` clones `&String` to `String`
- First char check uses `map` + `unwrap_or` for safety

**Usage:**
```rust
let names = vec!["Alice".to_string(), "bob".to_string(), "Charlie".to_string()];
let result: Vec<_> = uppercase_names(&names).collect();
// → vec!["Alice", "Charlie"]
```

## Exercise 5: Product with Fold

Calculate product using `fold`.

```rust
pub fn product_of_all(numbers: &[i32]) -> i32 {
    numbers.iter().fold(1, |acc, x| acc * x)
}
```

**Key Points:**
- `fold(1, ...)` starts with accumulator = 1
- First arg to closure is accumulator
- Second arg is current element
- Returns final accumulator value

**Comparison:**
```rust
// fold - most general, full control
.iter().fold(0, |sum, x| sum + x)

// sum - specialized for addition
.iter().sum()

// reduce - like fold but uses first element as start
.iter().reduce(|acc, x| acc * x)
```

## Exercise 6: Any and All

Check conditions across elements.

```rust
pub fn has_positive(numbers: &[i32]) -> bool {
    numbers.iter().any(|&x| x > 0)
}

pub fn all_positive(numbers: &[i32]) -> bool {
    numbers.iter().all(|&x| x > 0)
}
```

**Key Points:**
- `any()` returns `true` if at least one element matches
- `all()` returns `true` if all elements match
- Short-circuit evaluation: stop at first match/mismatch
- Empty iterator: `any()` returns `false`, `all()` returns `true`

## Iterator Adapters Reference

### Creating Iterators

| Adapter | Description | Example |
|---------|-------------|---------|
| `iter()` | References to elements | `vec.iter()` |
| `iter_mut()` | Mutable references | `vec.iter_mut()` |
| `into_iter()` | Takes ownership | `vec.into_iter()` |
| `drain(range)` | Remove and iterate | `vec.drain(..)` |

### Transformation

| Adapter | Description | Example |
|---------|-------------|---------|
| `map(f)` | Transform each | `.map(\|x\| x * 2)` |
| `filter(p)` | Keep matching | `.filter(\|x\| x > 0)` |
| `enumerate()` | Add indices | `.enumerate()` |
| `zip(other)` | Pair with other | `a.iter().zip(b.iter())` |
| `flat_map(f)` | Map then flatten | `.flat_map(\|x\| x.iter())` |
| `flatten()` | Flatten nested | `vec_of_vecs.flatten()` |

### Reduction

| Adapter | Description | Example |
|---------|-------------|---------|
| `collect()` | Gather into collection | `.collect::<Vec<_>>()` |
| `sum()` | Add all | `.sum::<i32>()` |
| `product()` | Multiply all | `.product::<i32>()` |
| `count()` | Count elements | `.count()` |
| `fold(i, f)` | Custom reduction | `.fold(0, \|a, b\| a + b)` |
| `reduce(f)` | Fold with first as init | `.reduce(\|a, b\| a + b)` |

### Short-circuiting

| Adapter | Description | Example |
|---------|-------------|---------|
| `any(p)` | True if any match | `.any(\|x\| x > 0)` |
| `all(p)` | True if all match | `.all(\|x\| x > 0)` |
| `find(p)` | First match | `.find(\|x\| x > 0)` |
| `position(p)` | Index of first match | `.position(\|x\| x > 0)` |
| `take(n)` | First n elements | `.take(5)` |
| `skip(n)` | Skip n elements | `.skip(5)` |

## Iterator Method Chains

```rust
// Complex processing pipeline
let result: Vec<_> = items
    .iter()
    .filter(|x| x.is_active)
    .map(|x| x.value * 2)
    .filter(|&x| x > 100)
    .take(10)
    .collect();

// Group and aggregate
let sum_by_category: HashMap<_, _> = items
    .iter()
    .map(|x| (x.category, x.amount))
    .fold(HashMap::new(), |mut map, (cat, amt)| {
        *map.entry(cat).or_insert(0) += amt;
        map
    });
```

## Tips

1. **Prefer iterator chains** over loops - More declarative, often faster
2. **Use `collect::<Vec<_>>()`** when type inference needs help
3. **Use `copied()` for Copy types** instead of `cloned()` - More efficient
4. **Consider `partition()`** to split into two collections
5. **`for_each()`** for side effects, but prefer `for` loop for clarity
6. **`peekable()`** for iterators that need look-ahead
7. **Parallel iterators with `rayon`** - Drop-in `par_iter()` for speed

## Common Idioms

```rust
// Find max by key
let max = items.iter().max_by_key(|x| x.score);

// Sort by multiple criteria
items.sort_by(|a, b| {
    b.score.cmp(&a.score)
        .then_with(|| a.name.cmp(&b.name))
});

// Chunk processing
for chunk in items.chunks(100) {
    process_batch(chunk);
}

// Windows for sliding view
for window in items.windows(3) {
    // Process 3 at a time
}
```
