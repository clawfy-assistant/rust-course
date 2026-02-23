//! # Module 07: Lifetimes - Advanced Examples
//!
//! Complex lifetime patterns and lifetime bounds

/// Lifetime elision in structs
/// Shows: struct lifetime annotations
pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }

    pub fn peek(&self) -> Option<&'a str> {
        self.input.get(self.position..self.position + 1)
    }

    pub fn consume(&mut self) -> Option<&'a str> {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    pub fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }
}

/// Multiple lifetimes for disambiguation
/// Shows: 'a, 'b, 'c in same function
pub fn longest_with_announcement<'a, 'b, T>(
    x: &'a str,
    y: &'b str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() >= y.len() { x } else { x }  // Always return x for 'a
}

/// Lifetime bounds on generics
/// Shows: T: 'a bound
pub struct BorrowedOrOwned<'a, T: 'a> {
    data: T,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> BorrowedOrOwned<'a, T> {
    pub fn new(data: T) -> Self {
        BorrowedOrOwned {
            data,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        &self.data
    }
}

/// Self-referential struct workaround
/// Shows: indices instead of references
pub struct TextBuffer {
    text: String,
    words: Vec<(usize, usize)>,  // (start, end) indices
}

impl TextBuffer {
    pub fn new(text: String) -> Self {
        let words = Self::find_words(&text);
        TextBuffer { text, words }
    }

    fn find_words(text: &str) -> Vec<(usize, usize)> {
        text.split_whitespace()
            .scan(0usize, |pos, word| {
                let start = text[*pos..].find(word)? + *pos;
                let end = start + word.len();
                *pos = end;
                Some((start, end))
            })
            .collect()
    }

    pub fn get_word(&self, index: usize) -> Option<&str> {
        self.words.get(index).map(|(start, end)| {
            &self.text[*start..*end]
        })
    }

    pub fn words(&self) -> impl Iterator<Item = &str> {
        self.words.iter().map(move |(s, e)| &self.text[*s..*e])
    }
}

/// Lifetime subtyping (covariance)
/// Shows: 'long: 'short
pub fn shorten_lifetime<'a, 'b: 'a>(long: &'b str) -> &'a str {
    long  // 'b outlives 'a, so this is safe
}

/// Higher-Ranked Trait Bounds (HRTB)
/// Shows: for<'a> syntax
pub trait ParserFn: for<'a> Fn(&'a str) -> &'a str {}
impl<T: for<'a> Fn(&'a str) -> &'a str> ParserFn for T {}

pub fn use_parser<F>(parser: F, input: &str) -> &str
where
    F: ParserFn,
{
    parser(input)
}

/// Static lifetime for global data
/// Shows: 'static, lazy_static equivalent
use std::sync::OnceLock;

pub fn get_global_config() -> &'static str {
    static CONFIG: OnceLock<String> = OnceLock::new();
    CONFIG.get_or_init(|| "default config".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut parser = Parser::new("Hello");
        assert_eq!(parser.peek(), Some("H"));
        assert_eq!(parser.consume(), Some("H"));
        assert_eq!(parser.remaining(), "ello");
    }

    #[test]
    fn test_text_buffer() {
        let buffer = TextBuffer::new("Hello world".to_string());
        assert_eq!(buffer.get_word(0), Some("Hello"));
        assert_eq!(buffer.get_word(1), Some("world"));
        
        let words: Vec<&str> = buffer.words().collect();
        assert_eq!(words, vec!["Hello", "world"]);
    }

    #[test]
    fn test_borrowed_or_owned() {
        let s = String::from("hello");
        let borrowed = BorrowedOrOwned::new(&s);
        assert_eq!(borrowed.get(), &"hello");

        let owned = BorrowedOrOwned::new(String::from("world"));
        assert_eq!(owned.get(), &"world");
    }

    #[test]
    fn test_hrtb() {
        let parser = |s: &str| &s[0..1];
        let result = use_parser(parser, "hello");
        assert_eq!(result, "h");
    }

    #[test]
    fn test_static_lifetime() {
        let config1 = get_global_config();
        let config2 = get_global_config();
        assert_eq!(config1, config2);
        // Both have 'static lifetime
    }
}