//! Lesson 04: Collections

use std::collections::HashMap;

/// นับความถี่ของคำใน text
pub fn word_frequency(text: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for word in text.to_lowercase().split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

/// หาค่าที่ซ้ำกันระหว่างสอง vector
pub fn find_duplicates(a: &[i32], b: &[i32]) -> Vec<i32> {
    use std::collections::HashSet;
    let set_a: HashSet<_> = a.iter().collect();
    b.iter()
        .filter(|x| set_a.contains(x))
        .copied()
        .collect()
}

/// รวม vector สองตัวเข้าด้วยกันแบบสลับ
pub fn interleave(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let min_len = a.len().min(b.len());
    
    for i in 0..min_len {
        result.push(a[i]);
        result.push(b[i]);
    }
    
    if a.len() > b.len() {
        result.extend_from_slice(&a[min_len..]);
    } else {
        result.extend_from_slice(&b[min_len..]);
    }
    
    result
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency() {
        let text = "hello world hello";
        let freq = word_frequency(text);
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&1));
    }

    #[test]
    fn test_find_duplicates() {
        assert_eq!(
            find_duplicates(&[1, 2, 3], &[2, 3, 4]),
            vec![2, 3]
        );
    }

    #[test]
    fn test_interleave() {
        assert_eq!(
            interleave(&[1, 2], &[10, 20, 30]),
            vec![1, 10, 2, 20, 30]
        );
    }
}
