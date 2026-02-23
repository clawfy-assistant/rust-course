# Solutions - Module 10: Concurrency

Complete solutions for threading and shared state.

## Exercise 1: Parallel Sum

Split work across multiple threads using `Arc` and `Mutex`.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_sum(numbers: Vec<i32>, num_threads: usize) -> i32 {
    let chunk_size = numbers.len() / num_threads;
    let numbers = Arc::new(Mutex::new(numbers));
    let mut handles = vec![];
    
    for i in 0..num_threads {
        let numbers = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let nums = numbers.lock().unwrap();
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                nums.len()
            } else {
                start + chunk_size
            };
            nums[start..end].iter().sum::<i32>()
        });
        handles.push(handle);
    }
    
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    total
}
```

**Key Points:**
- `Arc` (Atomic Reference Counting) allows shared ownership across threads
- `Mutex` provides mutual exclusion for safe mutable access
- `Arc::clone()` increments reference count (cheap)
- `lock()` acquires mutex, returns `MutexGuard`
- `join()` waits for thread to complete

**Thread spawning pattern:**
```rust
let data = Arc::new(Mutex::new(vec![1, 2, 3]));

for i in 0..num_threads {
    let data = Arc::clone(&data);  // Clone Arc, not data
    thread::spawn(move || {
        let mut guard = data.lock().unwrap();
        // guard auto-unlocks when dropped
    });
}
```

## Exercise 2: Thread-Safe Counter

A counter that can be shared and mutated across threads.

```rust
use std::sync::Mutex;

pub struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }
    
    pub fn increment(&self) {
        let mut v = self.value.lock().unwrap();
        *v += 1;
    }
    
    pub fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}
```

**Key Points:**
- `Mutex<i32>` inside struct protects the value
- Methods take `&self` (not `&mut self`) - interior mutability
- `lock().unwrap()` panics if mutex is poisoned
- `MutexGuard` dereferences to `&mut T`

**Better version with error handling:**
```rust
pub fn increment(&self) -> Result<(), String> {
    let mut v = self.value.lock()
        .map_err(|_| "Mutex poisoned")?;
    *v += 1;
    Ok(())
}
```

## Exercise 3: Channel Communication

Send data between threads using mpsc (multiple producer, single consumer).

```rust
pub fn send_and_receive() -> i32 {
    use std::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    
    rx.recv().unwrap()
}
```

**Key Points:**
- `mpsc::channel()` creates sender (tx) and receiver (rx)
- `send()` moves value to channel
- `recv()` blocks until message received
- `try_recv()` doesn't block, returns `Err` if empty

**Multiple producers:**
```rust
let (tx, rx) = mpsc::channel();

for i in 0..10 {
    let tx = tx.clone();  // Clone sender for each thread
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}

drop(tx);  // Drop original sender

for received in rx {
    println!("Got: {}", received);
}
```

## Concurrency Primitives

### Smart Pointers for Sharing

| Type | Use When | Characteristics |
|------|----------|-----------------|
| `Rc<T>` | Single-threaded sharing | Fast, not thread-safe |
| `Arc<T>` | Multi-threaded sharing | Atomic operations, slower |
| `RefCell<T>` | Single-threaded interior mutability | Runtime borrow checking |
| `Mutex<T>` | Thread-safe mutable access | Blocking, can deadlock |
| `RwLock<T>` | Many readers, few writers | Blocks writers or readers |

### Thread Management

```rust
// Spawn with return value
let handle = thread::spawn(|| {
    // work
    42  // return value
});
let result = handle.join().unwrap();

// Scoped threads (guaranteed to join)
thread::scope(|s| {
    s.spawn(|| {
        // thread automatically joins at scope end
    });
});

// Builder for named threads, stack size
thread::Builder::new()
    .name("worker".into())
    .stack_size(1024 * 1024)
    .spawn(|| { /* ... */ });
```

### Synchronization

```rust
// Barrier - wait for all threads
let barrier = Arc::new(Barrier::new(num_threads));
barrier.wait();  // Blocks until all threads reach here

// Condvar - condition variable
let (lock, cvar) = &*Arc::new((Mutex::new(false), Condvar::new()));
cvar.wait_while(lock.lock().unwrap(), |pending| !*pending);

// Atomic types - lock-free
use std::sync::atomic::{AtomicUsize, Ordering};
let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::SeqCst);
```

## Common Patterns

### Thread Pool Pattern

```rust
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
```

### Parallel Map

```rust
pub fn parallel_map<T, F, R>(input: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    let f = Arc::new(f);
    
    input.into_iter()
        .map(|item| {
            let f = Arc::clone(&f);
            thread::spawn(move || f(item))
        })
        .map(|handle| handle.join().unwrap())
        .collect()
}
```

## Tips

1. **Fearless concurrency** - Compiler catches data races at compile time
2. **Message passing > shared state** - Easier to reason about
3. **Use `rayon`** for data parallelism - Drop-in `par_iter()`
4. **Avoid `unwrap()` on locks** - Handle poisoned mutexes
5. **Use `std::sync::atomic`** for simple counters - Lock-free, faster
6. **Deadlocks happen** - Always acquire locks in same order
7. **Channel closing** - Drop all senders to signal done
8. **`tokio` for async** - Use async/await for I/O-bound work

## Send and Sync Traits

```rust
// Send: Safe to move to another thread
// Most types are Send

// Sync: Safe to share between threads (&T is Send)
// Most types are Sync

// NOT Send/Sync: Rc, RefCell, raw pointers
// ARE Send + Sync: Arc, Mutex, atomic types
```
