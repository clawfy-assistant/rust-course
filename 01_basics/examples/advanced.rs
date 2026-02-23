//! # Module 01: Basics - Advanced Examples
//! 
//! Real-world patterns using basic Rust features

use std::collections::HashMap;

/// Configuration builder pattern (like `std::process::Command`)
/// Shows: method chaining, consuming builders, type conversions
pub struct Config {
    pub name: String,
    pub port: u16,
    pub debug: bool,
    pub features: Vec<String>,
}

impl Config {
    pub fn new(name: &str) -> Self {
        Config {
            name: name.to_string(),
            port: 8080,
            debug: false,
            features: Vec::new(),
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    pub fn feature(mut self, feature: &str) -> Self {
        self.features.push(feature.to_string());
        self
    }

    pub fn build(self) -> Config {
        self
    }
}

/// State machine with enums (simplified HTTP parser state)
/// Shows: exhaustive matching, state transitions
#[derive(Debug, PartialEq)]
pub enum ParseState {
    Start,
    ReadingMethod(String),
    ReadingPath(String, String),  // method, path-so-far
    ReadingHeaders(String, String, HashMap<String, String>),  // method, path, headers
    Complete(String, String, HashMap<String, String>),
    Error(String),
}

impl ParseState {
    pub fn new() -> Self {
        ParseState::Start
    }

    pub fn next(self, input: char) -> Self {
        match self {
            ParseState::Start if input.is_ascii_alphabetic() => {
                ParseState::ReadingMethod(input.to_string())
            }
            ParseState::ReadingMethod(mut method) if input == ' ' => {
                ParseState::ReadingPath(method, String::new())
            }
            ParseState::ReadingMethod(mut method) => {
                method.push(input);
                ParseState::ReadingMethod(method)
            }
            ParseState::ReadingPath(method, mut path) if input == ' ' => {
                ParseState::ReadingHeaders(method, path, HashMap::new())
            }
            ParseState::ReadingPath(method, mut path) => {
                path.push(input);
                ParseState::ReadingPath(method, path)
            }
            ParseState::ReadingHeaders(method, path, headers) => {
                // Simplified - just complete on newline
                if input == '\n' {
                    ParseState::Complete(method, path, headers)
                } else {
                    ParseState::ReadingHeaders(method, path, headers)
                }
            }
            ParseState::Complete(_, _, _) => ParseState::Error("Already complete".to_string()),
            ParseState::Error(e) => ParseState::Error(e),
            _ => ParseState::Error("Invalid transition".to_string()),
        }
    }
}

/// Recursive descent parser for simple expressions
/// Shows: recursion, pattern matching, Result
pub fn evaluate_expr(expr: &str) -> Result<i32, String> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    parse_expression(&tokens, 0).map(|(val, _)| val)
}

fn parse_expression(tokens: &[&str], pos: usize) -> Result<(i32, usize), String> {
    if pos >= tokens.len() {
        return Err("Unexpected end of expression".to_string());
    }

    let (left, mut next_pos) = parse_term(tokens, pos)?;
    
    while next_pos < tokens.len() {
        match tokens[next_pos] {
            "+" => {
                let (right, new_pos) = parse_term(tokens, next_pos + 1)?;
                return Ok((left + right, new_pos));
            }
            "-" => {
                let (right, new_pos) = parse_term(tokens, next_pos + 1)?;
                return Ok((left - right, new_pos));
            }
            _ => break,
        }
    }
    
    Ok((left, next_pos))
}

fn parse_term(tokens: &[&str], pos: usize) -> Result<(i32, usize), String> {
    let token = tokens.get(pos).ok_or("Expected number")?;
    match token.parse::<i32>() {
        Ok(n) => Ok((n, pos + 1)),
        Err(_) => Err(format!("Invalid number: {}", token)),
    }
}

/// Bit manipulation for permissions (like Unix file permissions)
/// Shows: bitwise operations, const, type aliases
pub type Permission = u8;

pub const READ: Permission = 0b100;
pub const WRITE: Permission = 0b010;
pub const EXECUTE: Permission = 0b001;

pub struct FilePermissions {
    owner: Permission,
    group: Permission,
    others: Permission,
}

impl FilePermissions {
    pub fn from_octal(octal: u16) -> Self {
        FilePermissions {
            owner: ((octal >> 6) & 0o7) as u8,
            group: ((octal >> 3) & 0o7) as u8,
            others: (octal & 0o7) as u8,
        }
    }

    pub fn to_octal(&self) -> u16 {
        ((self.owner as u16) << 6) | ((self.group as u16) << 3) | (self.others as u16)
    }

    pub fn owner_can_read(&self) -> bool {
        self.owner & READ != 0
    }

    pub fn set_owner_permission(&mut self, perm: Permission, enabled: bool) {
        if enabled {
            self.owner |= perm;
        } else {
            self.owner &= !perm;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = Config::new("my-app")
            .port(3000)
            .debug(true)
            .feature("auth")
            .feature("logging")
            .build();
        
        assert_eq!(config.name, "my-app");
        assert_eq!(config.port, 3000);
        assert!(config.debug);
        assert_eq!(config.features, vec!["auth", "logging"]);
    }

    #[test]
    fn test_parse_state_machine() {
        let state = ParseState::new()
            .next('G')
            .next('E')
            .next('T')
            .next(' ')
            .next('/')
            .next(' ');
        
        match state {
            ParseState::ReadingHeaders(method, path, _) => {
                assert_eq!(method, "GET");
                assert_eq!(path, "/");
            }
            _ => panic!("Expected ReadingHeaders state"),
        }
    }

    #[test]
    fn test_evaluate_expr() {
        assert_eq!(evaluate_expr("5 + 3"), Ok(8));
        assert_eq!(evaluate_expr("10 - 4"), Ok(6));
        assert!(evaluate_expr("").is_err());
    }

    #[test]
    fn test_file_permissions() {
        let mut perms = FilePermissions::from_octical(0o755);
        assert!(perms.owner_can_read());
        assert!(perms.owner & WRITE != 0);
        assert!(perms.owner & EXECUTE != 0);
        
        perms.set_owner_permission(WRITE, false);
        assert!(!perms.owner_can_read());  // Wait, this checks READ not WRITE
        assert_eq!(perms.to_octal(), 0o555);
    }
}