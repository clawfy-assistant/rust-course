# Rust Learning Course 🦀

บทเรียน Rust สำหรับผู้เริ่มต้นถึงระดับกลาง พร้อมแบบฝึกหัดและเทสต์

## 📚 โครงสร้างบทเรียน

```
rust-course/
├── 01_basics/           # พื้นฐาน
├── 02_ownership/        # Ownership & Borrowing
├── 03_structs_enums/    # Structs & Enums
├── 04_collections/      # Vectors, HashMaps
├── 05_error_handling/   # Result & Option
├── 06_generics/         # Generics
├── 07_lifetimes/        # Lifetimes
├── 08_traits/           # Traits
├── 09_closures_iterators/# Closures & Iterators
├── 10_concurrency/      # Concurrency
├── 11_smart_pointers/   # Smart Pointers
├── 12_advanced/         # Advanced Topics
└── projects/            # โปรเจคจบ
```

## 🚀 เริ่มต้นใช้งาน

### ติดตั้ง Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### รันเทสต์ทั้งหมด
```bash
cargo test --workspace
```

### รันเฉพาะบทเรียน
```bash
cd 01_basics
cargo test
```

## 📖 บทเรียน

| # | หัวข้อ | ความยาก | เนื้อหา |
|---|-------|---------|---------|
| 01 | Basics | ⭐ | Variables, Types, Functions, Control Flow |
| 02 | Ownership | ⭐⭐ | Ownership, Borrowing, Slices |
| 03 | Structs & Enums | ⭐⭐ | Structs, Enums, Pattern Matching |
| 04 | Collections | ⭐⭐ | Vectors, HashMaps, Strings |
| 05 | Error Handling | ⭐⭐ | Result, Option, unwrap, ? |
| 06 | Generics | ⭐⭐⭐ | Generic Types, Functions |
| 07 | Lifetimes | ⭐⭐⭐ | Lifetime Annotations |
| 08 | Traits | ⭐⭐⭐ | Traits, Trait Bounds |
| 09 | Closures & Iterators | ⭐⭐⭐ | Closures, Iter, map, filter |
| 10 | Concurrency | ⭐⭐⭐⭐ | Threads, Channels, Arc, Mutex |
| 11 | Smart Pointers | ⭐⭐⭐⭐ | Box, Rc, RefCell |
| 12 | Advanced | ⭐⭐⭐⭐ | Unsafe, Macros, FFI |

## 📝 วิธีเรียน

1. อ่าน `README.md` ในแต่ละโฟลเดอร์
2. ดูตัวอย่างโค้ดใน `src/lib.rs`
3. ทำแบบฝึกหัดใน `src/exercises.rs`
4. รัน `cargo test` เพื่อเช็คคำตอบ
5. ถ้าผ่านทุกเทสต์ → ไปบทถัดไป!

## 🎯 โปรเจคจบ

- **Mini CLI Tool** - โปรแกรม command-line
- **Web Server** - HTTP server ง่ายๆ
- **Chat App** - แชทผ่าน TCP

## 📄 License

MIT License - ใช้เรียนและแจกจ่ายได้เสรี

Happy Rusting! 🦀✨
