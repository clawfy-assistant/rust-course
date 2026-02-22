//! # Lesson 03: Structs and Enums
//!
//! โครงสร้างข้อมูลและการจับคู่รูปแบบ

// ============================================
// EXERCISE 1: Structs
// ============================================

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// สร้าง Rectangle
    pub fn new(width: u32, height: u32) -> Self {
        // TODO: implement
        Rectangle { width: 0, height: 0 }
    }

    /// คำนวณพื้นที่
    pub fn area(&self) -> u32 {
        // TODO: คำนวณ width * height
        0
    }

    /// ตรวจสอบว่าเป็นสี่เหลี่ยมจัตุรัสหรือไม่
    pub fn is_square(&self) -> bool {
        // TODO: width == height
        false
    }

    /// ตรวจสอบว่าสี่เหลี่ยมนี้สามารถใส่ other ได้หรือไม่
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // TODO: self กว้างและยาวพอที่จะใส่ other
        false
    }
}

// ============================================
// EXERCISE 2: Enums
// ============================================

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// คืนค่าตรงกันข้าม
    pub fn opposite(&self) -> Direction {
        // TODO: North -> South, East -> West, etc.
        Direction::North
    }
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    /// เรียกเมื่อได้รับ message
    pub fn process(&self) -> String {
        // TODO: ใช้ match จัดการแต่ละแบบ
        // Quit -> "Quitting..."
        // Move -> "Moving to x, y"
        // Write -> "Writing: text"
        // ChangeColor -> "Changing color to r, g, b"
        String::new()
    }
}

// ============================================
// EXERCISE 3: Option
// ============================================

/// หา index ของตัวเลขใน vector
pub fn find_index(vec: &[i32], target: i32) -> Option<usize> {
    // TODO: คืน Some(index) ถ้าเจอ, None ถ้าไม่เจอ
    None
}

/// คืนค่าแรกของ vector ถ้ามี
pub fn first_element<T>(vec: &[T]) -> Option<&T> {
    // TODO: คืน Some(&first) ถ้ามี, None ถ้าว่าง
    None
}

/// บวกเลขสองตัวที่อาจเป็น None
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    // TODO: คืนผลบวกถ้าทั้งคู่มีค่า, ไม่งั้น None
    None
}

// ============================================
// EXERCISE 4: Result
// ============================================

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    InvalidNumber,
}

/// แปลงสตริงเป็น i32
pub fn parse_number(s: &str) -> Result<i32, ParseError> {
    // TODO: 
    // - ถ้าว่าง -> Err(ParseError::EmptyString)
    // - ถ้า parse ไม่ได้ -> Err(ParseError::InvalidNumber)
    // - ถ้าได้ -> Ok(number)
    Err(ParseError::EmptyString)
}

/// หารสองตัวเลข คืน error ถ้าหารด้วยศูนย์
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    // TODO: คืน Err ถ้า b == 0.0
    Ok(0.0)
}

/// อ่านค่าจาก Result หลายตัว
pub fn sum_results(results: Vec<Result<i32, &str>>) -> Result<i32, &str> {
    // TODO: บวกทุกตัวที่ Ok, คืน Err ทันทีถ้าเจอ Err
    Ok(0)
}

// ============================================
// EXERCISE 5: Advanced Pattern Matching
// ============================================

#[derive(Debug, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // รัฐที่ผลิต
}

/// คำนวณมูลค่าเหรียญเป็นเซ็นต์
pub fn coin_value(coin: &Coin) -> u8 {
    // TODO: ใช้ match
    // Penny = 1, Nickel = 5, Dime = 10, Quarter = 25
    0
}

/// นับจำนวน Quarter และรวมมูลค่า
pub fn count_quarters(coins: &[Coin]) -> (usize, u32) {
    // TODO: คืน (จำนวน Quarter, มูลค่ารวมทั้งหมด)
    (0, 0)
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
        assert!(!rect.is_square());

        let square = Rectangle::new(10, 10);
        assert!(square.is_square());
    }

    #[test]
    fn test_rectangle_can_hold() {
        let rect1 = Rectangle::new(30, 50);
        let rect2 = Rectangle::new(10, 20);
        let rect3 = Rectangle::new(40, 60);

        assert!(rect1.can_hold(&rect2));
        assert!(!rect1.can_hold(&rect3));
    }

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::West.opposite(), Direction::East);
    }

    #[test]
    fn test_message_process() {
        assert_eq!(Message::Quit.process(), "Quitting...");
        assert_eq!(
            Message::Move { x: 10, y: 20 }.process(),
            "Moving to 10, 20"
        );
        assert_eq!(
            Message::Write(String::from("Hello")).process(),
            "Writing: Hello"
        );
        assert_eq!(
            Message::ChangeColor(255, 0, 0).process(),
            "Changing color to 255, 0, 0"
        );
    }

    #[test]
    fn test_find_index() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(find_index(&vec, 3), Some(2));
        assert_eq!(find_index(&vec, 10), None);
    }

    #[test]
    fn test_first_element() {
        let vec = vec![1, 2, 3];
        assert_eq!(first_element(&vec), Some(&1));
        let empty: Vec<i32> = vec![];
        assert_eq!(first_element(&empty), None);
    }

    #[test]
    fn test_add_options() {
        assert_eq!(add_options(Some(5), Some(10)), Some(15));
        assert_eq!(add_options(Some(5), None), None);
        assert_eq!(add_options(None, Some(10)), None);
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number(""), Err(ParseError::EmptyString));
        assert_eq!(parse_number("abc"), Err(ParseError::InvalidNumber));
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_sum_results() {
        let results = vec![Ok(1), Ok(2), Ok(3)];
        assert_eq!(sum_results(results), Ok(6));

        let results = vec![Ok(1), Err("error"), Ok(3)];
        assert_eq!(sum_results(results), Err("error"));
    }

    #[test]
    fn test_coin_value() {
        assert_eq!(coin_value(&Coin::Penny), 1);
        assert_eq!(coin_value(&Coin::Nickel), 5);
        assert_eq!(coin_value(&Coin::Dime), 10);
        assert_eq!(coin_value(&Coin::Quarter(String::from("Alaska"))), 25);
    }

    #[test]
    fn test_count_quarters() {
        let coins = vec![
            Coin::Penny,
            Coin::Quarter(String::from("Alaska")),
            Coin::Quarter(String::from("Arizona")),
            Coin::Dime,
        ];
        assert_eq!(count_quarters(&coins), (2, 36)); // 2 quarters = 50, + 1 + 10 = 61
    }
}
