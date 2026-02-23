//! # Module 09: Closures and Iterators - Advanced Examples
//!
//! Functional programming patterns

use std::collections::HashMap;

/// Iterator adapter chains
/// Shows: map, filter, flat_map, collect
pub fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|n| n % 2 == 0)      // Keep only even numbers
        .map(|n| n * n)               // Square them
        .filter(|n| *n > 10)          // Keep only > 10
        .collect()
}

/// Lazy evaluation with iterators
/// Shows: infinite iterators, take, skip
pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut a = 0u64;
    let mut b = 1u64;
    
    std::iter::from_fn(move || {
        let current = a;
        a = b;
        b = current + b;
        Some(current)
    })
}

/// Grouping with fold
/// Shows: fold, entry API
pub fn group_by_length(words: Vec<String>) -> HashMap<usize, Vec<String>> {
    words.into_iter().fold(HashMap::new(), |mut acc, word| {
        acc.entry(word.len()).or_default().push(word);
        acc
    })
}

/// Partitioning
/// Shows: partition
pub fn separate_even_odd(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    numbers.into_iter().partition(|n| n % 2 == 0)
}

/// Finding with closures
/// Shows: find, position, any, all
pub fn find_first_positive(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().find(|n| *n > 0)
}

pub fn all_positive(numbers: &[i32]) -> bool {
    numbers.iter().all(|n| *n > 0)
}

/// Chaining iterators
/// Shows: chain, zip, enumerate
pub fn merge_lists<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().chain(b.iter()).cloned().collect()
}

pub fn pair_with_index<T: Clone>(items: &[T]) -> Vec<(usize, T)> {
    items.iter().enumerate().map(|(i, x)| (i, x.clone())).collect()
}

/// Closures capturing environment
/// Shows: move, Fn, FnMut, FnOnce
pub fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

pub fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

/// Higher-order functions
/// Shows: functions taking/returning functions
pub fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

pub fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| f(g(x))
}

/// Custom iterator adapter
/// Shows: extending Iterator trait
pub trait ChunkBy<T>: Iterator<Item = T> + Sized {
    fn chunk_by<F>(self, f: F) -> ChunkByIter<Self, T, F>
    where
        F: Fn(&T, &T) -> bool;
}

impl<T, I: Iterator<Item = T>> ChunkBy<T> for I {
    fn chunk_by<F>(self, f: F) -> ChunkByIter<Self, T, F>
    where
        F: Fn(&T, &T) -> bool,
    {
        ChunkByIter {
            iter: self,
            current: None,
            f,
        }
    }
}

pub struct ChunkByIter<I, T, F> {
    iter: I,
    current: Option<Vec<T>>,
    f: F,
}

impl<I, T, F> Iterator for ChunkByIter<I, T, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T, &T) -> bool,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(item) => {
                    if let Some(ref mut current) = self.current {
                        if (self.f)(¤t.last().unwrap(), &item) {
                            current.push(item);
                            continue;
                        } else {
                            return self.current.replace(vec![item]);
                        }
                    } else {
                        self.current = Some(vec![item]);
                        continue;
                    }
                }
                None => return self.current.take(),
            }
        }
    }
}

/// Iterator with state
/// Shows: scan, stateful iteration
pub fn running_sum(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_numbers() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let result = process_numbers(nums);
        assert_eq!(result, vec![16, 36]);  // 4²=16, 6²=36
    }

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = fibonacci().take(10).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_group_by_length() {
        let words = vec!["a".to_string(), "bb".to_string(), "cc".to_string(), "ddd".to_string()];
        let grouped = group_by_length(words);
        
        assert_eq!(grouped.get(&1).unwrap().len(), 1);
        assert_eq!(grouped.get(&2).unwrap().len(), 2);
        assert_eq!(grouped.get(&3).unwrap().len(), 1);
    }

    #[test]
    fn test_separate_even_odd() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let (even, odd) = separate_even_odd(nums);
        assert_eq!(even, vec![2, 4, 6]);
        assert_eq!(odd, vec![1, 3, 5]);
    }

    #[test]
    fn test_multiplier_closure() {
        let triple = make_multiplier(3);
        assert_eq!(triple(5), 15);
        assert_eq!(triple(10), 30);
    }

    #[test]
    fn test_counter_closure() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_apply_twice() {
        let double = |x| x * 2;
        assert_eq!(apply_twice(double, 5), 20);  // (5*2)*2
    }

    #[test]
    fn test_compose() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let add_then_double = compose(double, add_one);
        assert_eq!(add_then_double(5), 12);  // (5+1)*2
    }

    #[test]
    fn test_running_sum() {
        let nums = vec![1, 2, 3, 4, 5];
        let sums = running_sum(nums);
        assert_eq!(sums, vec![1, 3, 6, 10, 15]);
    }
}