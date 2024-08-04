//! Array helpers (currently unused)

use std::collections::VecDeque;

use super::models::manifest::SingleOrArray;

impl<T> SingleOrArray<T> {
    /// Get an iterator over the array
    pub fn iter(&self) -> NestedIterator<&T> {
        match self {
            SingleOrArray::Single(s) => NestedIterator::Single(Some(s)),
            SingleOrArray::Array(a) => NestedIterator::Array(a.iter().collect()),
        }
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        match self {
            SingleOrArray::Single(_) => 1,
            SingleOrArray::Array(a) => a.len(),
        }
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> IntoIterator for &'a SingleOrArray<T> {
    type Item = &'a T;
    type IntoIter = NestedIterator<&'a T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> IntoIterator for SingleOrArray<T> {
    type Item = T;
    type IntoIter = NestedIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SingleOrArray::Single(s) => NestedIterator::Single(Some(s)),
            SingleOrArray::Array(a) => NestedIterator::Array(a.into()),
        }
    }
}

#[derive(Debug, Clone)]
/// An iterator over a nested array
pub enum NestedIterator<T> {
    /// A single element
    Single(Option<T>),
    /// An array of elements
    Array(VecDeque<T>),
}

impl<T> Iterator for NestedIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(s) => std::mem::take(s),
            Self::Array(a) => a.pop_front(),
        }
    }
}
