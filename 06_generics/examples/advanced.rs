//! # Module 06: Generics - Advanced Examples
//!
//! Type-level programming and constraints

use std::cmp::Ordering;
use std::fmt::Debug;

/// Generic newtype pattern
/// Shows: tuple struct, Deref, From/Into
pub struct Meter(f64);
pub struct Kilometer(f64);

impl Meter {
    pub fn new(value: f64) -> Self {
        Meter(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl From<Kilometer> for Meter {
    fn from(km: Kilometer) -> Self {
        Meter(km.0 * 1000.0)
    }
}

impl From<Meter> for Kilometer {
    fn from(m: Meter) -> Self {
        Kilometer(m.0 / 1000.0)
    }
}

/// Phantom types for compile-time state
/// Shows: PhantomData, type-level state machines
use std::marker::PhantomData;

pub struct Empty;
pub struct Filled;

pub struct Container<T, State> {
    item: Option<T>,
    _state: PhantomData<State>,
}

impl<T> Container<T, Empty> {
    pub fn new() -> Self {
        Container {
            item: None,
            _state: PhantomData,
        }
    }

    pub fn fill(self, item: T) -> Container<T, Filled> {
        Container {
            item: Some(item),
            _state: PhantomData,
        }
    }
}

impl<T> Container<T, Filled> {
    pub fn get(&self) -> &T {
        self.item.as_ref().unwrap()
    }

    pub fn take(self) -> (T, Container<T, Empty>) {
        let item = self.item.unwrap();
        let empty = Container {
            item: None,
            _state: PhantomData,
        };
        (item, empty)
    }
}

/// Generic trait bounds with multiple constraints
/// Shows: where clauses, trait composition
pub fn process_items<T, I>(items: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
    T: Clone + Debug,
{
    items.into_iter().collect()
}

/// Generic associated types pattern
/// Shows: AssociatedType, GATs
pub trait ContainerTrait {
    type Item;
    type Iter: Iterator<Item = Self::Item>;

    fn items(&self) -> Self::Iter;
}

pub struct MyVec<T> {
    data: Vec<T>,
}

impl<T: Clone> ContainerTrait for MyVec<T> {
    type Item = T;
    type Iter = std::vec::IntoIter<T>;

    fn items(&self) -> Self::Iter {
        self.data.clone().into_iter()
    }
}

/// Const generics for fixed-size arrays
/// Shows: const N: usize, array types
pub struct FixedVector<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> FixedVector<T, N> {
    pub fn new() -> Self {
        FixedVector {
            data: [T::default(); N],
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &str> {
        if self.len >= N {
            return Err("Vector is full");
        }
        self.data[self.len] = item;
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }
}

/// Min-Max heap with custom comparators
/// Shows: generic comparison, Fn traits
pub fn find_extremes<T, F>(items: &[T], compare: F) -> Option<(&T, &T)>
where
    F: Fn(&T, &T) -> Ordering,
{
    if items.is_empty() {
        return None;
    }

    let mut min = &items[0];
    let mut max = &items[0];

    for item in &items[1..] {
        if compare(item, min) == Ordering::Less {
            min = item;
        }
        if compare(item, max) == Ordering::Greater {
            max = item;
        }
    }

    Some((min, max))
}

/// Default trait for generic initialization
/// Shows: Default, generic constraints
#[derive(Debug)]
pub struct Settings<T: Default> {
    value: T,
    enabled: bool,
}

impl<T: Default> Settings<T> {
    pub fn new() -> Self {
        Settings {
            value: T::default(),
            enabled: true,
        }
    }
}

impl<T: Default> Default for Settings<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_conversions() {
        let m = Meter::new(1500.0);
        let km: Kilometer = m.into();
        assert!((km.0 - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_phantom_container() {
        let empty = Container::<i32, Empty>::new();
        let filled = empty.fill(42);
        assert_eq!(*filled.get(), 42);
        
        let (item, empty) = filled.take();
        assert_eq!(item, 42);
        // empty is now Empty type again
    }

    #[test]
    fn test_fixed_vector() {
        let mut vec: FixedVector<i32, 3> = FixedVector::new();
        assert!(vec.push(1).is_ok());
        assert!(vec.push(2).is_ok());
        assert!(vec.push(3).is_ok());
        assert!(vec.push(4).is_err()); // Full
        
        assert_eq!(vec.get(1), Some(&2));
    }

    #[test]
    fn test_find_extremes() {
        let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let (min, max) = find_extremes(&numbers, |a, b| a.cmp(b)).unwrap();
        assert_eq!(*min, 1);
        assert_eq!(*max, 9);
    }

    #[test]
    fn test_default_settings() {
        let settings: Settings<i32> = Settings::default();
        assert_eq!(settings.value, 0);
        assert!(settings.enabled);
    }
}