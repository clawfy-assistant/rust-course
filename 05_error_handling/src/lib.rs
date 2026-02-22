//! Lesson 05: Error Handling with ? operator

use std::fs::File;
use std::io::{self, Read};

/// อ่านไฟล์ทั้งหมดเป็น String
pub fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// สร้าง Config จาก args
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename })
    }
}

/// หา lines ที่มี query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let args = vec![
            String::from("program"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "file.txt");
    }

    #[test]
    fn test_config_new_error() {
        let args = vec![String::from("program")];
        assert!(Config::new(&args).is_err());
    }

    #[test]
    fn test_search() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            search("Rust", contents),
            vec!["Rust:"]
        );
        assert_eq!(
            search("safe", contents),
            vec!["safe, fast, productive."]
        );
    }
}
