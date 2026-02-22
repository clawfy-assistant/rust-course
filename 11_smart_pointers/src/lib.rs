//! Lesson 11: Smart Pointers

use std::rc::Rc;
use std::cell::RefCell;

/// ใช้ Box สำหรับ recursive type
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> Self {
        List::Nil
    }
    
    pub fn prepend(self, elem: i32) -> Self {
        List::Cons(elem, Box::new(self))
    }
    
    pub fn sum(&self) -> i32 {
        match self {
            List::Cons(head, tail) => head + tail.sum(),
            List::Nil => 0,
        }
    }
}

/// ใช้ Rc สำหรับ shared ownership
pub struct SharedData {
    pub value: Rc<RefCell<i32>>,
}

impl SharedData {
    pub fn new(value: i32) -> Self {
        SharedData {
            value: Rc::new(RefCell::new(value)),
        }
    }
    
    pub fn clone_rc(&self) -> Rc<RefCell<i32>> {
        Rc::clone(&self.value)
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let list = List::new()
            .prepend(3)
            .prepend(2)
            .prepend(1);
        
        assert_eq!(list.sum(), 6);
    }

    #[test]
    fn test_shared_data() {
        let data = SharedData::new(5);
        let cloned = data.clone_rc();
        
        *cloned.borrow_mut() += 10;
        
        assert_eq!(*data.value.borrow(), 15);
    }
}
