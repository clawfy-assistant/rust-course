//! Lesson 09: Closures and Iterators

/// สร้าง closure ที่ capture environment
pub fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

/// Filter and map with iterators
pub fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&x| x > 0)
        .map(|x| x * 2)
        .collect()
}

/// หาผลรวมของเลขคู่
pub fn sum_of_evens(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .sum()
}

/// สร้าง iterator ของชื่อที่มีตัวพิมพ์ใหญ่
pub fn uppercase_names(names: &[String]) -> impl Iterator<Item = String> + '_ {
    names
        .iter()
        .filter(|name| name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
        .cloned()
}

/// ใช้ fold
pub fn product_of_all(numbers: &[i32]) -> i32 {
    numbers.iter().fold(1, |acc, x| acc * x)
}

/// ใช้ any และ all
pub fn has_positive(numbers: &[i32]) -> bool {
    numbers.iter().any(|&x| x > 0)
}

pub fn all_positive(numbers: &[i32]) -> bool {
    numbers.iter().all(|&x| x > 0)
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_multiplier() {
        let triple = make_multiplier(3);
        assert_eq!(triple(4), 12);
        assert_eq!(triple(5), 15);
    }

    #[test]
    fn test_process_numbers() {
        let nums = vec![-1, 2, -3, 4, 5];
        assert_eq!(process_numbers(&nums), vec![4, 8, 10]);
    }

    #[test]
    fn test_sum_of_evens() {
        assert_eq!(sum_of_evens(&[1, 2, 3, 4, 5, 6]), 12);
    }

    #[test]
    fn test_uppercase_names() {
        let names = vec![
            String::from("Alice"),
            String::from("bob"),
            String::from("Charlie"),
        ];
        let result: Vec<_> = uppercase_names(&names).collect();
        assert_eq!(result, vec!["Alice", "Charlie"]);
    }

    #[test]
    fn test_product_of_all() {
        assert_eq!(product_of_all(&[1, 2, 3, 4]), 24);
    }

    #[test]
    fn test_has_positive() {
        assert!(has_positive(&[-1, -2, 3]));
        assert!(!has_positive(&[-1, -2, -3]));
    }

    #[test]
    fn test_all_positive() {
        assert!(all_positive(&[1, 2, 3]));
        assert!(!all_positive(&[1, -2, 3]));
    }
}
