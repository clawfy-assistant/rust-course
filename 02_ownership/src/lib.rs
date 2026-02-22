//! # Lesson 02: Ownership and Borrowing
//!
//! แนวคิดที่สำคัญที่สุดใน Rust

// ============================================
// EXERCISE 1: Understanding Move
// ============================================

/// แก้ให้โค้ดนี้ทำงานได้โดยไม่ใช้ clone
pub fn ownership_demo() -> String {
    let s1 = String::from("Rust");
    let s2 = s1;
    // TODO: แก้ให้ return ค่าถูกต้อง
    s2
}

// ============================================
// EXERCISE 2: Borrowing
// ============================================

/// คืนความยาวของสตริงโดยไม่รับ ownership
/// Hint: ใช้ &String
pub fn get_length(s: String) -> usize {
    // TODO: แก้ signature และ implementation
    // ตอนนี้มันรับ ownership ไปเลย
    s.len()
}

/// ต่อสตริงเข้ากับ " World!" โดยไม่รับ ownership
/// คืนค่าสตริงใหม่
pub fn append_world(s: String) -> String {
    // TODO: รับ &str แทน และคืนค่า String ใหม่
    s
}

// ============================================
// EXERCISE 3: Mutable Borrowing
// ============================================

/// เพิ่ม "!" เข้าไปในสตริงที่รับมา (in-place)
/// Hint: ใช้ &mut String
pub fn exclaim(s: String) {
    // TODO: แก้ให้เพิ่ม ! เข้าไปใน s โดยตรง
}

/// สลับค่า a และ b (in-place)
pub fn swap_values(a: i32, b: i32) -> (i32, i32) {
    // TODO: ใช้ mutable reference สลับค่า
    (a, b)
}

// ============================================
// EXERCISE 4: String Slices
// ============================================

/// คืนคำแรกจากประโยค (จนถึงช่องว่างแรก)
/// ถ้าไม่มีช่องว่าง คืนทั้งสตริง
pub fn first_word(s: &str) -> &str {
    // TODO: หาช่องว่างแรก และคืน slice
    s
}

/// คืนคำสุดท้ายจากประโยค
pub fn last_word(s: &str) -> &str {
    // TODO: หาคำสุดท้าย
    s
}

/// คืน slice ตั้งแต่ตำแหน่ง start ถึง end
/// ถ้า out of bounds คืน ""
pub fn substring(s: &str, start: usize, end: usize) -> &str {
    // TODO: ตรวจสอบ bounds และคืน slice
    ""
}

// ============================================
// EXERCISE 5: Multiple Borrows
// ============================================

/// คืนผลรวมและค่าเฉลี่ยของ slice
/// Hint: ต้อง borrow หลายตัว
pub fn sum_and_average(numbers: &[i32]) -> (i32, f64) {
    // TODO: คำนวณผลรวมและค่าเฉลี่ย
    (0, 0.0)
}

/// หาค่า max และ min ใน slice
pub fn find_min_max(numbers: &[i32]) -> Option<(i32, i32)> {
    // TODO: คืน (min, max) ถ้าไม่ว่าง, None ถ้าว่าง
    None
}

// ============================================
// EXERCISE 6: Ownership with Structs
// ============================================

pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    /// สร้าง Person ใหม่
    pub fn new(name: &str, age: u32) -> Self {
        // TODO: สร้าง Person
        Person {
            name: String::new(),
            age,
        }
    }

    /// คืนชื่อโดยไม่ move
    pub fn get_name(&self) -> &str {
        // TODO: คืน &str
        &self.name
    }

    /// มีวันเกิด! (เพิ่มอายุ 1 ปี)
    pub fn have_birthday(&mut self) {
        // TODO: เพิ่มอายุ
    }
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_demo() {
        assert_eq!(ownership_demo(), "Rust");
    }

    #[test]
    fn test_get_length() {
        let s = String::from("hello");
        let len = get_length(s);
        // ถ้า get_length รับ ownership แบบเดิม, บรรทัดนี้จะ compile ไม่ผ่าน
        // แต่ถ้าแก้ให้ borrow ได้ถูกต้อง จะผ่าน
        // assert_eq!(s, "hello");  // ลอง uncomment เมื่อแก้เสร็จ
        assert_eq!(len, 5);
    }

    #[test]
    fn test_append_world() {
        let s = String::from("Hello");
        let result = append_world(s);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_exclaim() {
        let mut s = String::from("Hello");
        exclaim(s);
        // assert_eq!(s, "Hello!");  // แก้ให้ทำงานแบบ in-place
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_last_word() {
        assert_eq!(last_word("hello world"), "world");
        assert_eq!(last_word("rust is awesome"), "awesome");
        assert_eq!(last_word("single"), "single");
    }

    #[test]
    fn test_substring() {
        assert_eq!(substring("hello", 0, 2), "he");
        assert_eq!(substring("hello", 1, 4), "ell");
        assert_eq!(substring("hello", 10, 20), ""); // out of bounds
    }

    #[test]
    fn test_sum_and_average() {
        let nums = [1, 2, 3, 4, 5];
        let (sum, avg) = sum_and_average(&nums);
        assert_eq!(sum, 15);
        assert_eq!(avg, 3.0);
    }

    #[test]
    fn test_find_min_max() {
        assert_eq!(find_min_max(&[3, 1, 4, 1, 5]), Some((1, 5)));
        assert_eq!(find_min_max(&[10]), Some((10, 10)));
        assert_eq!(find_min_max(&[]), None);
    }

    #[test]
    fn test_person() {
        let mut person = Person::new("Alice", 25);
        assert_eq!(person.get_name(), "Alice");
        person.have_birthday();
        assert_eq!(person.age, 26);
    }
}
