//! # Module 12: Advanced Topics
//!
//! Unsafe Rust, macros, and FFI

/// Safe wrapper around unsafe code
/// Shows: unsafe blocks, raw pointers
pub struct UnsafeBuffer {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl UnsafeBuffer {
    pub fn with_capacity(cap: usize) -> Self {
        let layout = std::alloc::Layout::array::<u8>(cap).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        UnsafeBuffer { ptr, len: 0, cap }
    }

    pub fn push(&mut self, byte: u8) {
        if self.len >= self.cap {
            panic!("Buffer is full");
        }
        unsafe {
            *self.ptr.add(self.len) = byte;
        }
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.len {
            Some(unsafe { *self.ptr.add(index) })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for UnsafeBuffer {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::array::<u8>(self.cap).unwrap();
        unsafe {
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}

/// Unsafe trait for low-level operations
/// Shows: unsafe traits, unsafe impl
pub unsafe trait Zeroable {
    fn zeroed() -> Self;
}

unsafe impl Zeroable for u32 {
    fn zeroed() -> Self {
        0
    }
}

unsafe impl Zeroable for [u8; 4] {
    fn zeroed() -> Self {
        [0; 4]
    }
}

/// Declarative macros
/// Shows: macro_rules!, repetition
#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

/// Builder macro
#[macro_export]
macro_rules! builder {
    ($name:ident { $($field:ident: $ty:ty),* }) => {
        pub struct $name {
            $(
                pub $field: $ty,
            )*
        }

        impl $name {
            pub fn builder() -> $name {
                $name {
                    $($field: Default::default(),)*
                }
            }

            $(
                pub fn $field(mut self, value: $ty) -> Self {
                    self.$field = value;
                    self
                }
            )*
        }
    };
}

/// Unsafe transmutation
/// Shows: mem::transmute, union (safer alternative)
pub fn bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
    // Safer than: unsafe { std::mem::transmute::<[u8; 4], u32>(bytes) }
}

/// Inline assembly (nightly only, shown as concept)
/// Shows: asm! macro concept
#[cfg(feature = "nightly")]
pub unsafe fn read_timestamp() -> u64 {
    let mut low: u32;
    let mut high: u32;
    // asm!("rdtsc", out("eax") low, out("edx") high);
    ((high as u64) << 32) | (low as u64)
}

/// FFI example (concept)
/// Shows: extern "C", #[no_mangle]
#[cfg(feature = "ffi")]
mod ffi {
    #[no_mangle]
    pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
        a + b
    }
}

/// Type-level integers with const generics
/// Shows: const N: usize, array operations
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new() -> Self {
        Matrix {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if let Some(r) = self.data.get_mut(row) {
            if let Some(c) = r.get_mut(col) {
                *c = value;
            }
        }
    }
}

/// Compile-time assertions
/// Shows: const_assert macros
#[macro_export]
macro_rules! const_assert {
    ($x:expr) => {
        const _: () = assert!($x);
    };
}

// Usage: const_assert!(std::mem::size_of::<u32>() == 4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_buffer() {
        let mut buf = UnsafeBuffer::with_capacity(10);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        
        assert_eq!(buf.get(0), Some(1));
        assert_eq!(buf.get(1), Some(2));
        assert_eq!(buf.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_vec_of_strings_macro() {
        let v = vec_of_strings!("a", "b", "c");
        assert_eq!(v, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_hashmap_macro() {
        let map = hashmap! {
            "a" => 1,
            "b" => 2
        };
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }

    #[test]
    fn test_bytes_to_u32() {
        let bytes = [0x78, 0x56, 0x34, 0x12];
        let value = bytes_to_u32(bytes);
        assert_eq!(value, 0x12345678);  // Little endian
    }

    #[test]
    fn test_matrix() {
        let mut m: Matrix<i32, 3, 3> = Matrix::new();
        m.set(1, 1, 5);
        assert_eq!(m.get(1, 1), Some(&5));
        assert_eq!(m.get(3, 3), None);  // Out of bounds
    }

    #[test]
    fn test_zeroable_trait() {
        let z: u32 = Zeroable::zeroed();
        assert_eq!(z, 0);
        
        let arr: [u8; 4] = Zeroable::zeroed();
        assert_eq!(arr, [0, 0, 0, 0]);
    }
}