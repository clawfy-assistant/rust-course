//! # Module 05: Error Handling - Advanced Examples
//!
//! Professional error handling patterns

use std::fmt;
use std::error::Error;
use std::io;

/// ThisError-style derive macro simulation
/// Shows: std::error::Error impl, source(), Display
#[derive(Debug)]
pub enum AppError {
    Io { 
        context: String,
        source: io::Error 
    },
    Parse { 
        input: String,
        reason: String 
    },
    Config { 
        key: String,
        message: String 
    },
    Validation(Vec<ValidationError>),
}

#[derive(Debug)]
pub struct ValidationError {
    field: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io { context, source } => {
                write!(f, "IO error while {}: {}", context, source)
            }
            AppError::Parse { input, reason } => {
                write!(f, "Failed to parse '{}': {}", input, reason)
            }
            AppError::Config { key, message } => {
                write!(f, "Config error for '{}': {}", key, message)
            }
            AppError::Validation(errors) => {
                write!(f, "Validation failed with {} errors:", errors.len())?;
                for err in errors {
                    write!(f, "\n  - {}: {}", err.field, err.message)?;
                }
                Ok(())
            }
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AppError>;

/// Context extension trait (like anyhow::Context)
/// Shows: extension traits, consuming self
pub trait Context<T, E> {
    fn context(self, msg: &str) -> std::result::Result<T, AppError>;
    fn with_context<F>(self, f: F) -> std::result::Result<T, AppError>
    where
        F: FnOnce() -> String;
}

impl<T> Context<T, io::Error> for std::result::Result<T, io::Error> {
    fn context(self, msg: &str) -> std::result::Result<T, AppError> {
        self.map_err(|e| AppError::Io {
            context: msg.to_string(),
            source: e,
        })
    }

    fn with_context<F>(self, f: F) -> std::result::Result<T, AppError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| AppError::Io {
            context: f(),
            source: e,
        })
    }
}

/// Error accumulation (collect multiple errors)
/// Shows: Result -> Vec of errors
pub fn validate_user(name: &str, age: i32, email: &str) -> Result<()> {
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push(ValidationError {
            field: "name".to_string(),
            message: "Name cannot be empty".to_string(),
        });
    }

    if age < 0 || age > 150 {
        errors.push(ValidationError {
            field: "age".to_string(),
            message: "Age must be between 0 and 150".to_string(),
        });
    }

    if !email.contains('@') {
        errors.push(ValidationError {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(AppError::Validation(errors))
    }
}

/// Retry with exponential backoff
/// Shows: error handling in loops, sleep
pub fn with_retry<F, T>(mut f: F, max_attempts: u32) -> std::result::Result<T, AppError>
where
    F: FnMut() -> std::result::Result<T, AppError>,
{
    let mut last_error = None;
    
    for attempt in 1..=max_attempts {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_attempts {
                    let delay = std::time::Duration::from_millis(100 * 2_u64.pow(attempt - 1));
                    std::thread::sleep(delay);
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}

/// Error conversion with From trait
/// Shows: implicit conversions
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io {
            context: "operation".to_string(),
            source: err,
        }
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse {
            input: "".to_string(),
            reason: err.to_string(),
        }
    }
}

/// Using the ? operator with custom errors
pub fn read_config_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("reading config from {}", path))
}

pub fn parse_port(s: &str) -> Result<u16> {
    s.parse()
        .map_err(|_| AppError::Parse {
            input: s.to_string(),
            reason: "not a valid port number".to_string(),
        })
}

pub fn load_configuration(path: &str) -> Result<u16> {
    let content = read_config_file(path)?;
    let port_str = content.lines()
        .find(|line| line.starts_with("port="))
        .ok_or_else(|| AppError::Config {
            key: "port".to_string(),
            message: "Port not found in config".to_string(),
        })?;
    
    let port = parse_port(&port_str[5..])?;
    Ok(port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let err = AppError::Parse {
            input: "abc".to_string(),
            reason: "not a number".to_string(),
        };
        assert!(err.to_string().contains("abc"));
    }

    #[test]
    fn test_context_trait() {
        let result: std::result::Result<(), io::Error> = 
            Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        
        let with_context = result.context("loading config");
        assert!(with_context.is_err());
    }

    #[test]
    fn test_validate_user() {
        assert!(validate_user("Alice", 25, "alice@example.com").is_ok());
        
        let err = validate_user("", -5, "invalid").unwrap_err();
        match err {
            AppError::Validation(errors) => {
                assert_eq!(errors.len(), 3);
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_retry_success() {
        let mut attempts = 0;
        let result = with_retry(|| {
            attempts += 1;
            if attempts < 3 {
                Err(AppError::Config {
                    key: "test".to_string(),
                    message: "retry".to_string(),
                })
            } else {
                Ok(42)
            }
        }, 5);
        
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 3);
    }

    #[test]
    fn test_from_conversions() {
        let io_err = io::Error::new(io::ErrorKind::Other, "test");
        let app_err: AppError = io_err.into();
        
        match app_err {
            AppError::Io { .. } => (),  // Success
            _ => panic!("Expected Io error"),
        }
    }
}