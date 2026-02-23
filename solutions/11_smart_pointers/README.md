# Solutions - Module 11: Smart Pointers

Complete solutions for heap-allocated data and interior mutability.

## Exercise 1: Recursive List with Box

Implement a cons list using `Box` for heap allocation.

```rust
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> Self {
        List::Nil
    }
    
    pub fn prepend(self, elem: i32) -> Self {
        List::Cons(elem, Box::new(self))
    }
    
    pub fn sum(&self) -> i32 {
        match self {
            List::Cons(head, tail) => head + tail.sum(),
            List::Nil => 0,
        }
    }
}
```

**Key Points:**
- `Box<T>` allocates on the heap, pointer on stack
- Required for recursive types (compiler needs fixed size)
- `Box::new(value)` creates heap allocation
- Automatic dereferencing with `*` or method calls
- Dropped automatically when goes out of scope

**Visual:**
```
Stack          Heap
-----          ----
list ──────→ Cons(1, ┐)
                     │
                     ▼
                   Cons(2, ┐)
                            │
                            ▼
                          Nil
```

**Without Box (doesn't compile):**
```rust
enum BadList {
    Cons(i32, BadList),  // ERROR: infinite size!
    Nil,
}
```

## Exercise 2: Shared Mutable Data

Combine `Rc` and `RefCell` for shared ownership with interior mutability.

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub struct SharedData {
    pub value: Rc<RefCell<i32>>,
}

impl SharedData {
    pub fn new(value: i32) -> Self {
        SharedData {
            value: Rc::new(RefCell::new(value)),
        }
    }
    
    pub fn clone_rc(&self) -> Rc<RefCell<i32>> {
        Rc::clone(&self.value)
    }
}
```

**Key Points:**
- `Rc<T>`: Single-threaded reference counting
- `RefCell<T>`: Runtime borrow checking (interior mutability)
- Together: Multiple owners, mutable data
- `Rc::clone()` is cheap (increments count)
- `borrow()` / `borrow_mut()` for access

**Usage:**
```rust
let data = SharedData::new(5);
let cloned = data.clone_rc();

*cloned.borrow_mut() += 10;

assert_eq!(*data.value.borrow(), 15);
```

## Smart Pointers Comparison

| Pointer | Use Case | Thread-Safe | Mutable |
|---------|----------|-------------|---------|
| `Box<T>` | Heap allocation, recursive types | N/A | Yes (unique owner) |
| `Rc<T>` | Shared ownership | No | No |
| `Arc<T>` | Shared ownership across threads | Yes | No |
| `RefCell<T>` | Interior mutability | No | Yes (runtime checks) |
| `Mutex<T>` | Thread-safe mutability | Yes | Yes (blocking) |
| `RwLock<T>` | Many readers/writers | Yes | Yes (blocking) |
| `Cell<T>` | Interior mutability (Copy types) | No | Yes |

## Combining Smart Pointers

### Single-threaded Shared Mutability

```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(vec![1, 2, 3]));

let data2 = Rc::clone(&data);
data2.borrow_mut().push(4);

println!("{:?}", data.borrow());  // [1, 2, 3, 4]
```

### Multi-threaded Shared Mutability

```rust
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..10)
    .map(|_| {
        let data = Arc::clone(&data);
        std::thread::spawn(move || {
            let mut guard = data.lock().unwrap();
            *guard += 1;
        })
    })
    .collect();

for h in handles { h.join().unwrap(); }
assert_eq!(*data.lock().unwrap(), 10);
```

## Deref and Drop

### Custom Smart Pointer

```rust
use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}
```

**Deref coercion:** Rust automatically converts `&MyBox<T>` to `&T`

### Early Drop

```rust
let x = Box::new(42);
drop(x);  // Explicitly drop before end of scope
// x is no longer valid
```

## Common Patterns

### Interior Mutability Pattern

```rust
pub struct Config {
    settings: RefCell<HashMap<String, String>>,
}

impl Config {
    pub fn get(&self, key: &str) -> Option<String> {
        self.settings.borrow().get(key).cloned()
    }
    
    pub fn set(&self, key: &str, value: &str) {
        self.settings.borrow_mut().insert(key.to_string(), value.to_string());
    }
}
```

### Weak References (avoid cycles)

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// Weak references don't prevent dropping
// Upgrade Weak to Rc when needed
if let Some(parent) = node.parent.borrow().upgrade() {
    // parent is Rc<Node>
}
```

### Custom Drop for Resources

```rust
pub struct FileGuard {
    path: PathBuf,
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        // Cleanup: delete temp file, release lock, etc.
        let _ = std::fs::remove_file(&self.path);
    }
}
```

## Memory Layout

```rust
// Box<T>
Stack: pointer (8 bytes)
Heap:  T value

// Rc<T>
Stack: pointer (8 bytes)
Heap:  [strong: usize, weak: usize, T value]

// RefCell<T>
Stack: RefCell { borrow: Cell<isize>, value: UnsafeCell<T> }

// Vec<T>
Stack: [ptr, len, capacity] (24 bytes on 64-bit)
Heap:  [T, T, T, ...]
```

## Tips

1. **Use `Box` for recursion** - Only way to have infinite types
2. **Prefer `&T` over `Rc<T>`** - Borrow when possible
3. **`Arc` is expensive** - Don't use unless you need threads
4. **RefCell panics** - Not for production error handling
5. **Check with `try_borrow()`** - Avoid runtime panics
6. **Watch for cycles** - Use `Weak` for parent pointers
7. **`mem::drop` for early cleanup** - Explicit resource management
8. **`Cow<'a, T>`** for "clone on write" - Efficient when no mutation needed

## Choosing the Right Pointer

```
Need heap allocation?
├── Yes → Box<T>
│
Need shared ownership?
├── Single-threaded → Rc<T>
└── Multi-threaded → Arc<T>
    
Need mutability through shared reference?
├── Single-threaded → RefCell<T>
└── Multi-threaded → Mutex<T> or RwLock<T>

Common combinations:
├── Rc<RefCell<T>> - Shared mutable (single-thread)
├── Arc<Mutex<T>>  - Shared mutable (multi-thread)
└── Arc<RwLock<T>> - Shared, many readers (multi-thread)
```
