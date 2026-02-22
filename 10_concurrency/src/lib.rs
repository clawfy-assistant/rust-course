//! Lesson 10: Concurrency

use std::sync::{Arc, Mutex};
use std::thread;

/// สร้าง threads หลายตัวเพื่อคำนวณผลรวม
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

/// Counter ที่ thread-safe
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

/// ส่งข้อมูลผ่าน channel
pub fn send_and_receive() -> i32 {
    use std::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    
    rx.recv().unwrap()
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        let numbers: Vec<i32> = (1..=100).collect();
        let result = parallel_sum(numbers, 4);
        assert_eq!(result, 5050);
    }

    #[test]
    fn test_counter() {
        let counter = Counter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_send_and_receive() {
        assert_eq!(send_and_receive(), 42);
    }
}
