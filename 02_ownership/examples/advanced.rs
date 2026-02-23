//! # Module 02: Ownership - Advanced Examples
//!
//! Deep dive into Rust's ownership system

use std::mem;

/// Custom smart pointer with Drop trait
/// Shows: Drop, Deref, ownership transfer
pub struct SmartPtr<T> {
    data: Box<T>,
    name: String,
}

impl<T> SmartPtr<T> {
    pub fn new(data: T, name: &str) -> Self {
        println!("Creating SmartPtr: {}", name);
        SmartPtr {
            data: Box::new(data),
            name: name.to_string(),
        }
    }

    pub fn into_inner(self) -> T {
        println!("Extracting data from: {}", self.name);
        *self.data  // Dereference Box to get T
    }
}

impl<T> Drop for SmartPtr<T> {
    fn drop(&mut self) {
        println!("Dropping SmartPtr: {}", self.name);
    }
}

/// RAII (Resource Acquisition Is Initialization) pattern
/// Shows: guards, automatic cleanup
pub struct FileGuard {
    filename: String,
    is_open: bool,
}

impl FileGuard {
    pub fn open(filename: &str) -> Result<Self, String> {
        println!("Opening file: {}", filename);
        Ok(FileGuard {
            filename: filename.to_string(),
            is_open: true,
        })
    }

    pub fn read(&self) -> Result<String, String> {
        if !self.is_open {
            return Err("File is closed".to_string());
        }
        Ok(format!("Contents of {}", self.filename))
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        if self.is_open {
            println!("Auto-closing file: {}", self.filename);
            self.is_open = false;
        }
    }
}

/// Zero-copy string processing
/// Shows: lifetime elision, string slices
pub fn process_headers(headers: &str) -> Vec<(&str, &str)> {
    headers
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ':');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            Some((key, value))
        })
        .collect()
}

/// Self-referential struct workaround
/// Shows: Pin, unsafe (concept), why we need indices instead of references
pub struct Document {
    content: String,
    word_indices: Vec<(usize, usize)>,  // Store indices, not references!
}

impl Document {
    pub fn new(content: String) -> Self {
        let indices = Self::find_words(&content);
        Document {
            content,
            word_indices: indices,
        }
    }

    fn find_words(content: &str) -> Vec<(usize, usize)> {
        content
            .split_whitespace()
            .scan(0, |pos, word| {
                let start = content[*pos..].find(word).unwrap() + *pos;
                let end = start + word.len();
                *pos = end;
                Some((start, end))
            })
            .collect()
    }

    pub fn words(&self) -> impl Iterator<Item = &str> {
        self.word_indices
            .iter()
            .map(move |&(start, end)| &self.content[start..end])
    }
}

/// Cow (Clone on Write) for efficient string handling
/// Shows: avoiding unnecessary allocations
use std::borrow::Cow;

pub fn escape_html(input: &str) -> Cow<str> {
    if !input.contains(['&', '<', '>', '"']) {
        // No escaping needed - return borrowed reference
        Cow::Borrowed(input)
    } else {
        // Need to create new string
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                _ => result.push(c),
            }
        }
        Cow::Owned(result)
    }
}

/// Ownership transfer patterns
/// Shows: mem::take, mem::replace
pub struct ConnectionPool {
    connections: Vec<String>,
    max_size: usize,
}

impl ConnectionPool {
    pub fn new(max_size: usize) -> Self {
        ConnectionPool {
            connections: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn acquire(&mut self) -> Option<String> {
        // Take ownership from pool
        self.connections.pop()
    }

    pub fn release(&mut self, conn: String) {
        if self.connections.len() < self.max_size {
            self.connections.push(conn);
        }
        // Otherwise, conn is dropped
    }

    /// Steal all connections (useful for shutdown)
    pub fn drain(&mut self) -> Vec<String> {
        mem::take(&mut self.connections)
    }
}

/// Zero-cost abstractions with ownership
/// Shows: iterators don't allocate
pub fn find_common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str {
    let common_len = a.bytes()
        .zip(b.bytes())
        .take_while(|(x, y)| x == y)
        .count();
    
    // Safe because we counted valid UTF-8 bytes
    &a[..common_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_ptr_drop() {
        let ptr = SmartPtr::new(42, "test");
        let data = ptr.into_inner();
        assert_eq!(data, 42);
        // Drop runs here
    }

    #[test]
    fn test_file_guard() {
        {
            let file = FileGuard::open("test.txt").unwrap();
            let contents = file.read().unwrap();
            assert!(contents.contains("test.txt"));
            // File auto-closes here
        }
    }

    #[test]
    fn test_process_headers() {
        let headers = "Content-Type: application/json\nAuthorization: Bearer token";
        let parsed = process_headers(headers);
        assert_eq!(parsed[0], ("Content-Type", "application/json"));
        assert_eq!(parsed[1], ("Authorization", "Bearer token"));
    }

    #[test]
    fn test_document_words() {
        let doc = Document::new("Hello world test".to_string());
        let words: Vec<&str> = doc.words().collect();
        assert_eq!(words, vec!["Hello", "world", "test"]);
    }

    #[test]
    fn test_escape_html() {
        // No escaping needed - borrowed
        let result = escape_html("Hello");
        assert!(matches!(result, Cow::Borrowed(_)));
        
        // Escaping needed - owned
        let result = escape_html("<script>");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), "&lt;script&gt;");
    }

    #[test]
    fn test_connection_pool() {
        let mut pool = ConnectionPool::new(2);
        pool.release("conn1".to_string());
        pool.release("conn2".to_string());
        
        let conn = pool.acquire().unwrap();
        assert_eq!(conn, "conn2");
        
        let all = pool.drain();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_common_prefix() {
        assert_eq!(find_common_prefix("hello", "hey"), "he");
        assert_eq!(find_common_prefix("rust", "ruby"), "ru");
        assert_eq!(find_common_prefix("test", "abc"), "");
    }
}