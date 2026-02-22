//! Lesson 12: Advanced Topics

/// Unsafe: แปลง raw pointer
pub unsafe fn dangerous() -> i32 {
    42
}

/// ใช้ unsafe block
pub fn use_unsafe() -> i32 {
    unsafe {
        dangerous()
    }
}

/// สร้าง macro ง่าย
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

/// Type alias
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

/// Never type (diverging function)
pub fn never_returns() -> ! {
    loop {
        println!("forever");
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe() {
        assert_eq!(use_unsafe(), 42);
    }

    #[test]
    fn test_type_alias() {
        let distance: Kilometers = 100;
        assert_eq!(distance, 100);
    }
}
