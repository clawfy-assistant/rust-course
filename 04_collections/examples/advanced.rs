//! # Module 04: Collections - Advanced Examples
//!
//! Advanced vector and HashMap patterns

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};
use std::hash::{Hash, Hasher};

/// LRU Cache using VecDeque + HashMap
/// Shows: combining collections for complex data structures
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,  // Most recent at front
}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to front (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Update existing
            self.map.insert(key.clone(), value);
            self.order.retain(|k| k != &key);
        } else {
            // Evict if at capacity
            if self.map.len() >= self.capacity {
                if let Some(oldest) = self.order.pop_back() {
                    self.map.remove(&oldest);
                }
            }
            self.map.insert(key.clone(), value);
        }
        self.order.push_front(key);
    }
}

/// Group-by operation (like SQL GROUP BY)
/// Shows: entry API, or_insert
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut groups = HashMap::new();
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    groups
}

/// Multi-key index for fast lookups
/// Shows: multiple HashMaps, maintaining consistency
pub struct MultiIndex<T: Clone> {
    by_id: HashMap<u64, T>,
    by_name: HashMap<String, u64>,
    by_email: HashMap<String, u64>,
}

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl MultiIndex<User> {
    pub fn new() -> Self {
        MultiIndex {
            by_id: HashMap::new(),
            by_name: HashMap::new(),
            by_email: HashMap::new(),
        }
    }

    pub fn insert(&mut self, user: User) {
        self.by_name.insert(user.name.clone(), user.id);
        self.by_email.insert(user.email.clone(), user.id);
        self.by_id.insert(user.id, user);
    }

    pub fn get_by_id(&self, id: u64) -> Option<&User> {
        self.by_id.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&User> {
        self.by_name.get(name).and_then(|id| self.by_id.get(id))
    }

    pub fn get_by_email(&self, email: &str) -> Option<&User> {
        self.by_email.get(email).and_then(|id| self.by_id.get(id))
    }
}

/// Custom hasher for specific use cases
/// Shows: BuildHasher, faster hashing for integers
pub type FastMap<K, V> = HashMap<K, V, std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>;

/// Counting occurrences (frequency map)
/// Shows: entry().and_modify().or_insert()
pub fn count_occurrences<T: Eq + Hash>(items: &[T]) -> HashMap<&T, usize> {
    let mut counts = HashMap::new();
    for item in items {
        counts.entry(item)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    counts
}

/// Finding duplicates using HashSet
/// Shows: HashSet operations
pub fn find_duplicates<T: Eq + Hash + Clone>(items: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    
    for item in items {
        if !seen.insert(item.clone()) {
            duplicates.insert(item.clone());
        }
    }
    
    duplicates.into_iter().collect()
}

/// Top-K elements using BTreeMap
/// Shows: sorted collections, range queries
pub fn top_k_by_frequency(words: Vec<String>, k: usize) -> Vec<(String, usize)> {
    let counts = count_occurrences(&words);
    
    // Invert: frequency -> list of words
    let mut by_freq: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for (word, count) in counts {
        by_freq.entry(count).or_default().push(word.clone());
    }
    
    // Get top k (highest frequencies first)
    let mut result = Vec::new();
    for (freq, words) in by_freq.iter().rev() {
        for word in words {
            if result.len() >= k {
                break;
            }
            result.push((word.clone(), *freq));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(&1));
        
        cache.put("c", 3);  // Should evict "b"
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_group_by() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let grouped = group_by(items, |x| x % 2);
        
        assert_eq!(grouped.get(&0), Some(&vec![2, 4, 6]));
        assert_eq!(grouped.get(&1), Some(&vec![1, 3, 5]));
    }

    #[test]
    fn test_multi_index() {
        let mut index = MultiIndex::new();
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        
        index.insert(user);
        
        assert!(index.get_by_id(1).is_some());
        assert!(index.get_by_name("Alice").is_some());
        assert!(index.get_by_email("alice@example.com").is_some());
    }

    #[test]
    fn test_count_occurrences() {
        let items = vec!["a", "b", "a", "c", "a", "b"];
        let counts = count_occurrences(&items);
        
        assert_eq!(counts.get(&"a"), Some(&3));
        assert_eq!(counts.get(&"b"), Some(&2));
        assert_eq!(counts.get(&"c"), Some(&1));
    }

    #[test]
    fn test_find_duplicates() {
        let items = vec![1, 2, 3, 2, 4, 3, 5];
        let dups = find_duplicates(&items);
        
        assert!(dups.contains(&2));
        assert!(dups.contains(&3));
        assert!(!dups.contains(&1));
    }

    #[test]
    fn test_top_k() {
        let words = vec![
            "rust".to_string(),
            "rust".to_string(),
            "rust".to_string(),
            "java".to_string(),
            "java".to_string(),
            "go".to_string(),
        ];
        
        let top = top_k_by_frequency(words, 2);
        assert_eq!(top[0], ("rust".to_string(), 3));
        assert_eq!(top[1], ("java".to_string(), 2));
    }
}