//! Lesson 07: Lifetimes

/// คืนค่าสตริงที่ยาวที่สุด (ใช้ lifetime ร่วมกัน)
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

/// Struct ที่มี reference (ต้องมี lifetime)
#[derive(Debug)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        Excerpt { part }
    }
    
    pub fn get_part(&self) -> &str {
        self.part
    }
}

/// หาคำแรก (ไม่ต้อง lifetime เพราะรับ &str เข้า คืน &str ออก)
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let s1 = String::from("long string is long");
        let s2 = "xyz";
        assert_eq!(longest(s1.as_str(), s2), s1.as_str());
    }

    #[test]
    fn test_excerpt() {
        let text = String::from("hello world");
        let excerpt = Excerpt::new(&text[..5]);
        assert_eq!(excerpt.get_part(), "hello");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
    }
}
