//! Lesson 06: Generics

/// สลับค่า generic สองตัว
pub fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

/// หาค่ามากสุดใน slice (ต้อง implement Ord)
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

/// Generic Point
#[derive(Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        assert_eq!(swap(5, 10), (10, 5));
        assert_eq!(swap("a", "b"), ("b", "a"));
    }

    #[test]
    fn test_largest() {
        let numbers = vec![1, 5, 3, 10, 2];
        assert_eq!(largest(&numbers), Some(&10));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(largest(&empty), None);
    }

    #[test]
    fn test_point() {
        let p = Point::new(5, 10);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 10);
        
        let p_f64 = Point::new(3.0, 4.0);
        assert_eq!(p_f64.distance_from_origin(), 5.0);
    }
}
