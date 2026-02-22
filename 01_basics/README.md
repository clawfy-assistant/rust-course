# 01 - พื้นฐาน Rust ⭐

## เนื้อหา

### 1.1 Variables และ Mutability
- `let` - immutable (ค่าเปลี่ยนไม่ได้)
- `let mut` - mutable (ค่าเปลี่ยนได้)
- Constants ด้วย `const`

```rust
let x = 5;           // immutable
let mut y = 10;      // mutable
const MAX: i32 = 100; // constant
```

### 1.2 Data Types
- **Scalar**: `i32`, `f64`, `bool`, `char`
- **Compound**: `tuple`, `array`

```rust
let tup: (i32, f64, bool) = (500, 6.4, true);
let arr = [1, 2, 3, 4, 5];
```

### 1.3 Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // expression (ไม่มี ;)
}
```

### 1.4 Control Flow
- `if`, `else if`, `else`
- `loop`, `while`, `for`

```rust
if x > 5 {
    println!("big");
} else {
    println!("small");
}

for i in 0..5 {
    println!("{}", i);
}
```

## 📝 แบบฝึกหัด

เปิดไฟล์ `src/lib.rs` และทำให้ทุกเทสต์ผ่าน!

## 🌍 Real-World Examples

### ใช้ในชีวิตจริง

**Configuration Management**
```rust
const MAX_CONNECTIONS: u32 = 100;
const DEFAULT_PORT: u16 = 8080;

fn main() {
    let mut current_connections = 0;
    while current_connections < MAX_CONNECTIONS {
        // รับ connection ใหม่
        current_connections += 1;
    }
}
```

**Data Processing Pipeline**
```rust
fn process_batch(data: &[i32]) -> Vec<i32> {
    let mut results = Vec::new();
    for value in data {
        if *value > 0 {
            results.push(value * 2);
        }
    }
    results
}
```

**CLI Argument Parsing**
```rust
fn parse_args(args: &[String]) -> (String, u32) {
    let command = &args[1];
    let count: u32 = args[2].parse().unwrap_or(1);
    (command.clone(), count)
}
```

### ใช้ในโปรเจคไหนบ้าง
- **Config files**: อ่านค่าคงที่ เช่น API keys, ports
- **CLI tools**: Argument parsing, command handling
- **Data validation**: Input checking, range validation

## ✅ รันเทสต์

```bash
cargo test
```
