//! # Module 08: Traits - Advanced Examples
//!
//! Advanced trait patterns and object-oriented design

use std::fmt;

/// Trait inheritance
/// Shows: trait bounds on other traits
pub trait Animal: fmt::Debug {
    fn name(&self) -> &str;
    fn make_sound(&self) -> String;
}

pub trait Mammal: Animal {
    fn walk(&self);
}

pub trait Bird: Animal {
    fn fly(&self);
}

#[derive(Debug)]
pub struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
}

impl Mammal for Dog {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

/// Trait objects with dyn
/// Shows: dynamic dispatch, object safety
pub fn animal_concert(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{} says: {}", animal.name(), animal.make_sound());
    }
}

/// Associated types vs generics
/// Shows: Output associated type
pub trait Convertible {
    type Output;
    fn convert(&self) -> Self::Output;
}

impl Convertible for i32 {
    type Output = String;
    fn convert(&self) -> String {
        self.to_string()
    }
}

impl Convertible for String {
    type Output = usize;
    fn convert(&self) -> usize {
        self.len()
    }
}

/// Default trait implementations
/// Shows: default methods, override
pub trait Drawable {
    fn draw(&self);
    
    fn describe(&self) -> String {
        format!("A drawable object")
    }
}

pub struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }

    fn describe(&self) -> String {
        format!("A circle with radius {}", self.radius)
    }
}

/// Operator overloading with traits
/// Shows: Add, Mul, etc.
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Iterator trait implementation
/// Shows: IntoIterator, custom iterators
pub struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }
}

impl IntoIterator for Range {
    type Item = i32;
    type IntoIter = RangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        RangeIterator {
            current: self.start,
            end: self.end,
        }
    }
}

pub struct RangeIterator {
    current: i32,
    end: i32,
}

impl Iterator for RangeIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let val = self.current;
            self.current += 1;
            Some(val)
        } else {
            None
        }
    }
}

/// Blanket implementations
/// Shows: impl<T: Trait> OtherTrait for T
pub trait Printable {
    fn print(&self);
}

impl<T: fmt::Display> Printable for T {
    fn print(&self) {
        println!("{}", self);
    }
}

/// Trait aliases (stable equivalent)
pub trait Numeric: Add<Output = Self> + Mul<Output = Self> + Copy {}
impl<T> Numeric for T where T: Add<Output = T> + Mul<Output = T> + Copy {}

fn double<T: Numeric>(x: T) -> T {
    x + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal_traits() {
        let dog = Dog { name: "Buddy".to_string() };
        assert_eq!(dog.make_sound(), "Woof!");
        dog.walk();
    }

    #[test]
    fn test_animal_concert() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog { name: "Buddy".to_string() }),
        ];
        animal_concert(animals);
    }

    #[test]
    fn test_convertible() {
        let num: i32 = 42;
        assert_eq!(num.convert(), "42");
        
        let s = String::from("hello");
        assert_eq!(s.convert(), 5);
    }

    #[test]
    fn test_drawable() {
        let circle = Circle { radius: 5.0 };
        assert!(circle.describe().contains("circle"));
    }

    #[test]
    fn test_point_operators() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let sum = p1 + p2;
        assert_eq!(sum, Point::new(4.0, 6.0));

        let scaled = p1 * 2.0;
        assert_eq!(scaled, Point::new(2.0, 4.0));
    }

    #[test]
    fn test_custom_iterator() {
        let range = Range::new(1, 5);
        let collected: Vec<i32> = range.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_printable_blanket() {
        let num = 42;
        num.print();  // Works because i32 implements Display
        
        let text = String::from("hello");
        text.print();  // Works because String implements Display
    }

    #[test]
    fn test_numeric_trait() {
        assert_eq!(double(5), 10);
        assert_eq!(double(3.5), 7.0);
    }
}