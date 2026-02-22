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

## ✅ รันเทสต์

```bash
cargo test
```
