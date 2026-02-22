//! # Lesson 01: Basics
//! 
//! บทเรียนพื้นฐาน: Variables, Types, Functions, Control Flow

// ============================================
// EXERCISE 1: Variables and Mutability
// ============================================

/// เปลี่ยน x ให้เป็น mutable และบวก 10 เข้าไป
/// คืนค่าสุดท้ายของ x
pub fn make_mutable_and_add_ten() -> i32 {
    let x = 5;
    // TODO: แก้ให้ x เป็น mutable และบวก 10
    
    x
}

/// สลับค่า a และ b โดยใช้ tuple destructuring
pub fn swap(a: i32, b: i32) -> (i32, i32) {
    // TODO: สลับค่า a และ b
    (a, b)
}

// ============================================
// EXERCISE 2: Data Types
// ============================================

/// คำนวณพื้นที่วงกลม (πr²) โดยใช้ r = 2.5
/// π ใช้ค่า std::f64::consts::PI
pub fn circle_area() -> f64 {
    let r = 2.5;
    // TODO: คำนวณพื้นที่
    0.0
}

/// คืนค่าตัวอักษรแรกและตัวเลขจาก tuple
pub fn get_first_char_and_number(tup: (char, i32, bool)) -> (char, i32) {
    // TODO: ดึงค่า char และ i32 จาก tuple
    ('a', 0)
}

/// คืนค่าผลรวมของ array [10, 20, 30, 40, 50]
pub fn sum_array() -> i32 {
    let arr = [10, 20, 30, 40, 50];
    // TODO: คำนวณผลรวม
    0
}

// ============================================
// EXERCISE 3: Functions
// ============================================

/// คืนค่าที่มากกว่าระหว่าง a และ b
pub fn max_of_two(a: i32, b: i32) -> i32 {
    // TODO: คืนค่าที่มากกว่า
    0
}

/// คำนวณ factorial (n!) 
/// 0! = 1, 1! = 1, 2! = 2, 3! = 6, ...
pub fn factorial(n: u32) -> u32 {
    // TODO: คำนวณ factorial
    1
}

/// ตรวจสอบว่าเป็นจำนวนเฉพาะหรือไม่
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    // TODO: ตรวจสอบจำนวนเฉพาะ
    true
}

// ============================================
// EXERCISE 4: Control Flow - FizzBuzz
// ============================================

/// FizzBuzz: 
/// - หาร 3 ลงตัว → "Fizz"
/// - หาร 5 ลงตัว → "Buzz"
/// - หารทั้งคู่ → "FizzBuzz"
/// - อื่นๆ → ตัวเลขเป็น string
pub fn fizzbuzz(n: i32) -> String {
    // TODO: implement FizzBuzz
    n.to_string()
}

// ============================================
// EXERCISE 5: Loops
// ============================================

/// คืนค่าผลรวมของเลข 1 ถึง n โดยใช้ for loop
pub fn sum_to_n(n: i32) -> i32 {
    // TODO: ใช้ for loop คำนวณผลรวม
    0
}

/// หาเลข fibonacci ตัวที่ n (0, 1, 1, 2, 3, 5, 8, ...)
pub fn fibonacci(n: u32) -> u32 {
    // TODO: หา fibonacci
    0
}

/// นับจำนวนตัวอักษรในสตริง (ไม่รับช่องว่าง)
pub fn count_chars(s: &str) -> usize {
    // TODO: นับตัวอักษรที่ไม่ใช่ช่องว่าง
    0
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_mutable_and_add_ten() {
        assert_eq!(make_mutable_and_add_ten(), 15);
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap(5, 10), (10, 5));
        assert_eq!(swap(-1, 1), (1, -1));
    }

    #[test]
    fn test_circle_area() {
        let result = circle_area();
        assert!((result - 19.6349).abs() < 0.001);
    }

    #[test]
    fn test_get_first_char_and_number() {
        assert_eq!(get_first_char_and_number(('A', 42, true)), ('A', 42));
    }

    #[test]
    fn test_sum_array() {
        assert_eq!(sum_array(), 150);
    }

    #[test]
    fn test_max_of_two() {
        assert_eq!(max_of_two(5, 10), 10);
        assert_eq!(max_of_two(20, 10), 20);
        assert_eq!(max_of_two(5, 5), 5);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(18), false);
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
    }

    #[test]
    fn test_sum_to_n() {
        assert_eq!(sum_to_n(5), 15);  // 1+2+3+4+5
        assert_eq!(sum_to_n(10), 55);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("hello"), 5);
        assert_eq!(count_chars("hello world"), 10); // ไม่นับช่องว่าง
        assert_eq!(count_chars("  rust  "), 4);
    }
}
