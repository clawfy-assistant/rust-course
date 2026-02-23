//! # Module 10: Concurrency - Advanced Examples
//!
//! Real-world async and parallel patterns

use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::Duration;

/// Actor pattern with message passing
/// Shows: mpsc, thread spawning, graceful shutdown
pub struct Actor<T> {
    sender: mpsc::Sender<Message<T>>,
    handle: thread::JoinHandle<()>,
}

enum Message<T> {
    Work(T),
    Stop,
}

impl<T: Send + 'static> Actor<T> {
    pub fn new<F>(mut processor: F) -> Self
    where
        F: FnMut(T) + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel::<Message<T>>();
        
        let handle = thread::spawn(move || {
            while let Ok(msg) = receiver.recv() {
                match msg {
                    Message::Work(item) => processor(item),
                    Message::Stop => break,
                }
            }
            println!("Actor shutting down gracefully");
        });
        
        Actor { sender, handle }
    }

    pub fn send(&self, item: T) -> Result<(), mpsc::SendError<Message<T>>> {
        self.sender.send(Message::Work(item))
    }

    pub fn stop(self) {
        let _ = self.sender.send(Message::Stop);
        let _ = self.handle.join();
    }
}

/// Thread pool with work stealing concept
/// Shows: Arc<Mutex>, condition variables, thread management
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down thread pool");
        // Drop sender to close channel
        drop(self.sender.take());
        
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {} executing job", id);
                        job();
                    }
                    Err(_) => {
                        println!("Worker {} shutting down", id);
                        break;
                    }
                }
            }
        });
        
        Worker {
            id,
            handle: Some(handle),
        }
    }
}

/// Read-heavy data structure with RwLock
/// Shows: multiple readers, single writer
pub struct Cache<K, V> {
    data: RwLock<std::collections::HashMap<K, V>>,
}

impl<K: Eq + std::hash::Hash, V: Clone> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            data: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        // Many threads can hold read lock simultaneously
        let map = self.data.read().unwrap();
        map.get(key).cloned()
    }

    pub fn insert(&self, key: K, value: V) {
        // Only one thread can hold write lock
        let mut map = self.data.write().unwrap();
        map.insert(key, value);
    }
}

/// Atomic operations without locks
/// Shows: AtomicUsize, memory ordering
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counter {
    count: AtomicUsize,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst)
    }

    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

/// Scoped threads (Rust 1.63+)
/// Shows: borrowing data into threads safely
pub fn parallel_sum(data: &[usize]) -> usize {
    if data.len() < 100 {
        return data.iter().sum();
    }
    
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    
    // Safe because threads are joined before function returns
    thread::scope(|s| {
        let left_handle = s.spawn(|| left.iter().sum::<usize>());
        let right_sum: usize = right.iter().sum();
        
        left_handle.join().unwrap() + right_sum
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;

    #[test]
    fn test_actor() {
        let actor = Actor::new(|msg: String| {
            println!("Processed: {}", msg);
        });
        
        actor.send("Hello".to_string()).unwrap();
        actor.send("World".to_string()).unwrap();
        
        thread::sleep(Duration::from_millis(10));
        actor.stop();
    }

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(Mutex::new(0));
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut c = counter.lock().unwrap();
                *c += 1;
            });
        }
        
        thread::sleep(Duration::from_millis(100));
        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_cache() {
        let cache = Cache::new();
        cache.insert("key1", "value1".to_string());
        cache.insert("key2", "value2".to_string());
        
        assert_eq!(cache.get(&"key1"), Some("value1".to_string()));
        assert_eq!(cache.get(&"key2"), Some("value2".to_string()));
    }

    #[test]
    fn test_counter() {
        let counter = Arc::new(Counter::new());
        let mut handles = vec![];
        
        for _ in 0..10 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    c.increment();
                }
            }));
        }
        
        for h in handles {
            h.join().unwrap();
        }
        
        assert_eq!(counter.get(), 1000);
    }

    #[test]
    fn test_parallel_sum() {
        let data: Vec<usize> = (0..1000).collect();
        let sum = parallel_sum(&data);
        assert_eq!(sum, data.iter().sum::<usize>());
    }

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = vec![];
        
        for i in 0..3 {
            let b = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                println!("Thread {} before barrier", i);
                b.wait();
                println!("Thread {} after barrier", i);
            }));
        }
        
        for h in handles {
            h.join().unwrap();
        }
    }
}