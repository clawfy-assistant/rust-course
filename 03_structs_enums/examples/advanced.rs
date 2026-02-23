//! # Module 03: Structs & Enums - Advanced Examples
//!
//! Type-safe state machines and error handling

use std::collections::HashMap;
use std::fmt;

/// Type-state pattern for compile-time state checking
/// Shows: phantom types, consuming methods
pub struct Idle;
pub struct Running;
pub struct Stopped;

pub struct Machine<State> {
    name: String,
    _state: std::marker::PhantomData<State>,
}

impl Machine<Idle> {
    pub fn new(name: &str) -> Self {
        Machine {
            name: name.to_string(),
            _state: std::marker::PhantomData,
        }
    }

    pub fn start(self) -> Machine<Running> {
        println!("Starting {}", self.name);
        Machine {
            name: self.name,
            _state: std::marker::PhantomData,
        }
    }
}

impl Machine<Running> {
    pub fn process(&self, data: &str) -> String {
        format!("{} processing: {}", self.name, data)
    }

    pub fn stop(self) -> Machine<Stopped> {
        println!("Stopping {}", self.name);
        Machine {
            name: self.name,
            _state: std::marker::PhantomData,
        }
    }
}

impl Machine<Stopped> {
    pub fn restart(self) -> Machine<Idle> {
        println!("Restarting {}", self.name);
        Machine {
            name: self.name,
            _state: std::marker::PhantomData,
        }
    }
}

/// Never type pattern for unrepresentable states
/// Shows: ! type (never), exhaustive matching
#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_path(&self, path: &str) -> Option<&JsonValue> {
        let mut current = self;
        for key in path.split('.') {
            match current {
                JsonValue::Object(map) => {
                    current = map.get(key)?;
                }
                _ => return None,
            }
        }
        Some(current)
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// ThisError-style error handling
/// Shows: enums with data, #[derive(Debug)]
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed { host: String, port: u16 },
    QueryFailed { sql: String, reason: String },
    NotFound { table: String, id: i64 },
    ConstraintViolation { field: String, value: String },
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed { host, port } => {
                write!(f, "Failed to connect to {}:{}", host, port)
            }
            DatabaseError::QueryFailed { sql, reason } => {
                write!(f, "Query failed: {} - SQL: {}", reason, sql)
            }
            DatabaseError::NotFound { table, id } => {
                write!(f, "Record {} not found in table {}", id, table)
            }
            DatabaseError::ConstraintViolation { field, value } => {
                write!(f, "Constraint violation on field '{}' with value '{}'", field, value)
            }
        }
    }
}

impl std::error::Error for DatabaseError {}

/// Result type alias for cleaner signatures
pub type DbResult<T> = Result<T, DatabaseError>;

/// Using Result in real code
pub fn fetch_user(id: i64) -> DbResult<User> {
    if id < 0 {
        return Err(DatabaseError::NotFound {
            table: "users".to_string(),
            id,
        });
    }
    Ok(User { id, name: "Alice".to_string() })
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
}

/// Non-exhaustive enum (extensible API)
#[derive(Debug)]
#[non_exhaustive]
pub enum StatusCode {
    Ok,
    NotFound,
    ServerError,
    // More variants can be added in future
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::ServerError => 500,
            // _ => 0,  // Required if not #[non_exhaustive]
        }
    }
}

/// Visitor pattern with enums
pub trait Visitor {
    fn visit_int(&mut self, value: i64);
    fn visit_string(&mut self, value: &str);
    fn visit_bool(&mut self, value: bool);
}

pub struct DebugVisitor;

impl Visitor for DebugVisitor {
    fn visit_int(&mut self, value: i64) {
        println!("Int: {}", value);
    }

    fn visit_string(&mut self, value: &str) {
        println!("String: {}", value);
    }

    fn visit_bool(&mut self, value: bool) {
        println!("Bool: {}", value);
    }
}

/// Enum with methods that return different types
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "integer",
            Value::Float(_) => "float",
            Value::Text(_) => "text",
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            Value::Text(t) => t.parse().ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_state() {
        let machine = Machine::<Idle>::new("Processor");
        let running = machine.start();
        let result = running.process("data");
        assert!(result.contains("data"));
        let stopped = running.stop();
        let _idle = stopped.restart();
        // Can't call stop() on idle or running on stopped - compile time check!
    }

    #[test]
    fn test_json_value() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), JsonValue::String("Alice".to_string()));
        obj.insert("age".to_string(), JsonValue::Number(30.0));
        
        let json = JsonValue::Object(obj);
        assert_eq!(json.get_path("name").and_then(|v| v.as_string()), Some("Alice"));
        
        let display = format!("{}", json);
        assert!(display.contains("name"));
        assert!(display.contains("Alice"));
    }

    #[test]
    fn test_database_error() {
        let err = DatabaseError::NotFound {
            table: "users".to_string(),
            id: 42,
        };
        assert!(err.to_string().contains("users"));
        assert!(err.to_string().contains("42"));
    }

    #[test]
    fn test_fetch_user() {
        let user = fetch_user(1).unwrap();
        assert_eq!(user.id, 1);
        
        let err = fetch_user(-1).unwrap_err();
        assert!(matches!(err, DatabaseError::NotFound { .. }));
    }

    #[test]
    fn test_value_conversion() {
        let int = Value::Int(42);
        let float = Value::Float(3.14);
        let text = Value::Text("100".to_string());
        
        assert_eq!(int.as_int(), Some(42));
        assert_eq!(float.as_int(), Some(3));  // Truncated
        assert_eq!(text.as_int(), Some(100)); // Parsed
    }
}