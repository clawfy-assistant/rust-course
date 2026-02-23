# Solutions - Module 04: Collections

Complete solutions for vector, HashMap, and HashSet operations.

## Exercise 1: Word Frequency

Count occurrences of each word in a text using `HashMap`.

```rust
use std::collections::HashMap;

pub fn word_frequency(text: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for word in text.to_lowercase().split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}
```

**Key Points:**
- `HashMap::new()` creates an empty map
- `entry()` gets or inserts a key
- `or_insert(0)` returns mutable reference to value (0 if new)
- `*map.entry(...)` dereferences to modify the value
- `split_whitespace()` handles multiple spaces automatically

**Usage:**
```rust
let text = "hello world hello";
let freq = word_frequency(text);
// freq = {"hello": 2, "world": 1}
```

## Exercise 2: Find Duplicates

Find common elements between two slices using `HashSet`.

```rust
use std::collections::HashSet;

pub fn find_duplicates(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<_> = a.iter().collect();
    b.iter()
        .filter(|x| set_a.contains(x))
        .copied()
        .collect()
}
```

**Key Points:**
- `HashSet` for O(1) lookups
- `collect()` converts iterator to collection
- `filter()` keeps elements matching predicate
- `copied()` copies values from `&i32` to `i32`
- Double references (`**x`) in filter due to iterator nesting

**Optimization:** 
- Build set from smaller slice for better memory usage
- Use `cloned()` for non-Copy types

## Exercise 3: Interleave Vectors

Merge two vectors by alternating elements.

```rust
pub fn interleave(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let min_len = a.len().min(b.len());
    
    // Interleave common length
    for i in 0..min_len {
        result.push(a[i]);
        result.push(b[i]);
    }
    
    // Append remaining elements from longer vector
    if a.len() > b.len() {
        result.extend_from_slice(&a[min_len..]);
    } else {
        result.extend_from_slice(&b[min_len..]);
    }
    
    result
}
```

**Key Points:**
- `min()` compares lengths safely
- `extend_from_slice()` efficiently appends slice
- Slicing `&a[min_len..]` gets remaining elements
- Works with unequal length vectors

**Example:**
```rust
interleave(&[1, 2], &[10, 20, 30])  // → [1, 10, 2, 20, 30]
```

## Collection Types Summary

| Type | Use When | Key Methods |
|------|----------|-------------|
| `Vec<T>` | Ordered list, indexed access | `push()`, `iter()`, `extend()` |
| `HashMap<K, V>` | Key-value lookup | `insert()`, `get()`, `entry()` |
| `HashSet<T>` | Unique items, membership test | `insert()`, `contains()` |
| `VecDeque<T>` | Queue operations | `push_back()`, `pop_front()` |
| `BTreeMap<K, V>` | Sorted key-value pairs | Same as HashMap |

## Tips

1. **Pre-allocate capacity** when size is known: `Vec::with_capacity(n)`
2. **Entry API** is powerful: `map.entry(key).or_insert_with(|| expensive())`
3. **Iterator chains** are efficient - Rust optimizes them well
4. **Use references** in collections to avoid cloning: `HashSet<&T>` instead of `HashSet<T>`
5. **Choose HashSet over Vec** for membership testing (O(1) vs O(n))

## Common Patterns

```rust
// Group by key
let groups: HashMap<K, Vec<V>> = items
    .into_iter()
    .fold(HashMap::new(), |mut map, item| {
        map.entry(item.key).or_default().push(item);
        map
    });

// Count with HashMap
let counts = items.iter().map(|x| x.category)
    .fold(HashMap::new(), |mut map, cat| {
        *map.entry(cat).or_insert(0) += 1;
        map
    });

// Remove duplicates while preserving order
let unique: Vec<_> = items.into_iter()
    .scan(HashSet::new(), |seen, x| {
        if seen.insert(x.id) { Some(x) } else { None }
    })
    .collect();
```
