//! # Module 11: Smart Pointers - Advanced Examples
//!
//! Custom smart pointers and interior mutability patterns

use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// Custom Box-like smart pointer
/// Shows: Deref, DerefMut, Drop
pub struct MyBox<T> {
    data: *mut T,  // Raw pointer for manual allocation
}

impl<T> MyBox<T> {
    pub fn new(data: T) -> Self {
        let ptr = Box::into_raw(Box::new(data));
        MyBox { data: ptr }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.data));
        }
    }
}

/// Reference counting with interior mutability
/// Shows: Rc<RefCell> pattern for shared mutable state
pub struct SharedState {
    data: Rc<RefCell<Vec<i32>>>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn add(&self, value: i32) {
        self.data.borrow_mut().push(value);
    }

    pub fn count(&self) -> usize {
        self.data.borrow().len()
    }

    pub fn clone_ref(&self) -> Self {
        SharedState {
            data: Rc::clone(&self.data),
        }
    }
}

/// Weak references to prevent cycles
/// Shows: Rc::downgrade, Weak::upgrade
pub struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Option<std::rc::Weak<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        })
    }

    pub fn add_child(self: &Rc<Self>, child: &Rc<Node>) {
        *child.parent.borrow_mut() = Some(Rc::downgrade(self));
        self.children.borrow_mut().push(Rc::clone(child));
    }

    pub fn parent_value(&self) -> Option<i32> {
        self.parent
            .borrow()
            .as_ref()
            .and_then(|weak| weak.upgrade())
            .map(|parent| parent.value)
    }
}

/// Memory-efficient storage with Cow
/// Shows: Clone-on-Write for zero-copy when possible
use std::borrow::Cow;

pub struct ConfigValue {
    data: Cow<'static, str>,
}

impl ConfigValue {
    pub fn from_static(s: &'static str) -> Self {
        ConfigValue {
            data: Cow::Borrowed(s),
        }
    }

    pub fn from_string(s: String) -> Self {
        ConfigValue {
            data: Cow::Owned(s),
        }
    }

    pub fn set(&mut self, value: String) {
        self.data = Cow::Owned(value);
    }

    pub fn as_str(&self) -> &str {
        &self.data
    }
}

/// Lazy initialization with OnceCell
/// Shows: thread-safe lazy static equivalent
use std::sync::OnceLock;

pub struct Settings;

impl Settings {
    pub fn get_config() -> &'static Config {
        static CONFIG: OnceLock<Config> = OnceLock::new();
        CONFIG.get_or_init(|| Config {
            database_url: "postgres://localhost".to_string(),
            max_connections: 10,
        })
    }
}

pub struct Config {
    database_url: String,
    max_connections: u32,
}

/// Pin for self-referential structs
/// Shows: Pin, why we need it for async
use std::pin::Pin;

pub struct SelfReferential {
    data: String,
    // pointer: *const String,  // Would point to data - unsafe!
}

impl SelfReferential {
    pub fn new(data: String) -> Pin<Box<Self>> {
        Box::pin(SelfReferential { data })
    }

    pub fn data(self: Pin<&Self>) -> &str {
        &self.get_ref().data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_box() {
        let mut b = MyBox::new(5);
        assert_eq!(*b, 5);
        *b = 10;
        assert_eq!(*b, 10);
    }

    #[test]
    fn test_shared_state() {
        let state1 = SharedState::new();
        let state2 = state1.clone_ref();
        
        state1.add(1);
        state2.add(2);
        
        assert_eq!(state1.count(), 2);
        assert_eq!(state2.count(), 2);
    }

    #[test]
    fn test_node_tree() {
        let root = Node::new(1);
        let child = Node::new(2);
        let grandchild = Node::new(3);
        
        root.add_child(&child);
        child.add_child(&grandchild);
        
        assert_eq!(child.parent_value(), Some(1));
        assert_eq!(grandchild.parent_value(), Some(2));
        
        // Drop root - tree is cleaned up properly
        // No cycles thanks to Weak references
    }

    #[test]
    fn test_config_value() {
        let mut config = ConfigValue::from_static("default");
        assert!(matches!(config.data, Cow::Borrowed(_)));
        
        config.set("custom".to_string());
        assert!(matches!(config.data, Cow::Owned(_)));
        
        assert_eq!(config.as_str(), "custom");
    }

    #[test]
    fn test_lazy_settings() {
        let config1 = Settings::get_config();
        let config2 = Settings::get_config();
        
        // Same instance
        assert_eq!(config1.max_connections, config2.max_connections);
    }

    #[test]
    fn test_self_referential() {
        let sr = SelfReferential::new("hello".to_string());
        assert_eq!(sr.data(), "hello");
    }

    #[test]
    fn test_arc_clone() {
        let data = Arc::new(vec![1, 2, 3]);
        let data2 = Arc::clone(&data);
        
        assert_eq!(Arc::strong_count(&data), 2);
        
        drop(data2);
        assert_eq!(Arc::strong_count(&data), 1);
    }
}