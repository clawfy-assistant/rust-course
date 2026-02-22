# 02 - Ownership & Borrowing ⭐⭐

## 🔑 แนวคิดสำคัญ

Rust ไม่มี garbage collector แต่ใช้ **Ownership** เพื่อจัดการหน่วยความจำ

### กฎ 3 ข้อของ Ownership

1. ทุก value มี **owner** คนเดียว
2. มีได้ owner ได้แค่ **หนึ่งคน** ในหนึ่งเวลา
3. เมื่อ owner ออกจาก scope, value จะถูก **drop**

### การย้าย Ownership (Move)

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 ถูก move ไปยัง s2
// println!("{}", s1);  // ❌ Error! s1 ใช้ไม่ได้แล้ว
```

### Borrowing (&, &mut)

```rust
fn main() {
    let s = String::from("hello");
    
    let len = calculate_length(&s);  // ยืม immutable
    println!("{} {}", s, len);       // ✅ ใช้ s ได้ต่อ
}

fn calculate_length(s: &String) -> usize {
    s.len()  // ไม่ได้เป็นเจ้าของ แค่ยืมมาใช้
}
```

### Slices

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // slice ของสตริง
```

## 📝 แบบฝึกหัด

ทำให้โค้ด compile ผ่านโดยไม่ใช้ `.clone()` ถ้าไม่จำเป็น!

## 🌍 Real-World Use Cases

### 🔥 ทำไม Ownership สำคัญ?

**Systems Programming (Linux Kernel, Embedded)**
```rust
// อ่านไฟล์โดยไม่ copy ข้อมูลซ้ำ
fn read_config(path: &str) -> Result<&str, Error> {
    let contents = fs::read_to_string(path)?;
    // ประมวลผลโดยใช้ borrow ไม่ต้อง clone
    parse_config(&contents)
}
```

**Web Servers (Actix, Axum)**
```rust
// Request handler ที่รับ reference ไม่ต้อง clone request
async fn handler(req: &Request) -> Response {
    let user_id = req.header("user-id");  // borrow
    let user = db.find_user(user_id).await;
    // req ยังใช้ต่อได้หลังจากนี้
}
```

**Game Development (Bevy, Amethyst)**
```rust
// อ่านข้อมูล entity หลายๆ ระบบพร้อมกัน
fn update_position(positions: &mut [Position], velocities: &[Velocity]) {
    for (pos, vel) in positions.iter_mut().zip(velocities.iter()) {
        pos.x += vel.dx;  // mutable borrow
    }
    // velocities ยังอ่านได้ (immutable borrow)
}
```

**Network Protocols (Tokio, async-std)**
```rust
// Zero-copy parsing ไม่ allocate หน่วยความจำใหม่
fn parse_packet(data: &[u8]) -> Packet {
    Packet {
        header: &data[0..4],      // slice - borrow
        payload: &data[4..],       // slice - borrow
    }
}
```

**Database Connections (SQLx, Diesel)**
```rust
// Connection pool - ยืม connection ไม่ต้องสร้างใหม่ทุกครั้ง
async fn get_user(pool: &Pool<Postgres>, id: i32) -> User {
    let mut conn = pool.acquire().await.unwrap();  // borrow from pool
    sqlx::query_as::<User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&mut conn).await
        .unwrap()
}  // connection คืน pool อัตโนมัติ
```

### 💡 บริษัทที่ใช้จริง

| บริษัท | ใช้ทำอะไร |
|--------|-----------|
| **Discord** | Voice server รองรับ millions of concurrent connections |
| **Cloudflare** | Edge proxy ประมวลผล HTTP ล้าน request/วินาที |
| **Figma** | Multiplayer sync engine แบบ real-time |

### ⚡ Performance Impact

```rust
// ❌ แบบที่ copy เยอะ
fn process(data: String) -> String {
    let copy1 = data.clone();
    let copy2 = copy1.clone();
    copy2
}

// ✅ แบบใช้ borrow - เร็วกว่า 100x
fn process_fast(data: &str) -> &str {
    data  // ไม่ copy อะไรเลย!
}
```

**Memory usage ต่างกันมาก:**
- Clone String 1MB = ใช้หน่วยความจำเพิ่ม 1MB
- Borrow &str = ใช้ 16 bytes (pointer + length)

### 🎯 สรุป

Ownership ทำให้ Rust:
- ✅ **Memory safe** - ไม่มี dangling pointer, use-after-free
- ✅ **Zero-cost abstractions** - ไม่มี runtime overhead
- ✅ **High performance** - ไม่ต้อง garbage collect
- ✅ **Concurrent-safe** - borrow checker ป้องกัน data race ตอน compile
