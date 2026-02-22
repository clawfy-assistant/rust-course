# 03 - Structs & Enums ⭐⭐

## Structs

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("joe@example.com"),
    username: String::from("joe"),
    active: true,
    sign_in_count: 1,
};
```

## Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Pattern Matching

```rust
match message {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Write: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
}
```

## Option & Result

```rust
// Option - ค่าอาจมีหรือไม่มี
let some_number = Some(5);
let absent_number: Option<i32> = None;

// Result - สำหรับ error handling
let result: Result<i32, String> = Ok(42);
```
