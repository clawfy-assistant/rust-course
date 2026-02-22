# 05 - Error Handling ⭐⭐

## Result & Option, unwrap, ? operator

## 🌍 Real-World Use Cases

### **API Clients**
```rust
// HTTP request with proper error handling
async fn fetch_user(api: &ApiClient, id: u32) -> Result<User, ApiError> {
    let response = api.get(format!("/users/{}", id)).await?;
    
    if response.status() == 404 {
        return Err(ApiError::UserNotFound);
    }
    
    let user: User = response.json().await?;
    Ok(user)
}
```
**ใช้โดย:** reqwest, hyper, tonic (gRPC)

### **Database Operations**
```rust
fn get_user_by_email(conn: &DbConnection, email: &str) -> Result<User, DbError> {
    let user = users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .map_err(|e| match e {
            diesel::NotFound => DbError::UserNotFound,
            _ => DbError::ConnectionFailed,
        })?;
    
    Ok(user)
}
```
**ใช้โดย:** SQLx, Diesel, SeaORM

### **Configuration Loading**
```rust
#[derive(Debug)]
enum ConfigError {
    FileNotFound(String),
    InvalidFormat(String),
    MissingField(String),
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = fs::read_to_string(path)
        .map_err(|_| ConfigError::FileNotFound(path.to_string()))?;
    
    let config: Config = toml::from_str(&contents)
        .map_err(|e| ConfigError::InvalidFormat(e.to_string()))?;
    
    // Validate required fields
    if config.database_url.is_empty() {
        return Err(ConfigError::MissingField("database_url".to_string()));
    }
    
    Ok(config)
}
```

### **File Processing**
```rust
fn process_csv_file(path: &Path) -> Result<Vec<Record>, CsvError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let record = parse_record(&line)
            .map_err(|e| CsvError::ParseError {
                line: line_num + 1,
                message: e.to_string(),
            })?;
        records.push(record);
    }
    
    Ok(records)
}
```

## 💡 Best Practices จาก Production

### **ThisError vs Anyhow**
```rust
// Library code: ใช้ thiserror (define error types)
#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

// Application code: ใช้ anyhow (convenient error handling)
fn main() -> Result<(), anyhow::Error> {
    let config = std::fs::read_to_string("config.toml")?;
    let app = App::new(&config)?;
    app.run()?;
    Ok(())
}
```

### **Early Returns**
```rust
// ❌ ซ้อนกันเยอะ
fn process(data: &str) -> Result<Vec<i32>, Error> {
    match parse_data(data) {
        Ok(items) => {
            match validate_items(&items) {
                Ok(valid) => {
                    match transform(valid) {
                        Ok(result) => Ok(result),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// ✅ ใช้ ? operator
fn process_fast(data: &str) -> Result<Vec<i32>, Error> {
    let items = parse_data(data)?;
    let valid = validate_items(&items)?;
    transform(valid)  // ไม่ต้อง ? ตัวสุดท้าย
}
```

## 📊 Error Handling ใน Production

| Approach | เหมาะกับ | Example |
|----------|----------|---------|
| `Result<T, E>` | Expected errors | File not found, invalid input |
| `panic!` | Unrecoverable bugs | Programming errors, invariant violation |
| `Option<T>` | Optional values | Cache miss, optional field |

## 📝 แบบฝึกหัด

ดูโค้ดใน `src/lib.rs`