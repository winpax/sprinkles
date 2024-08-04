//! Array helpers (currently unused)

use super::models::manifest::SingleOrArray;

impl<T> SingleOrArray<T> {
    /// Get an iterator over the array
    pub fn iter(&self) -> TOrArrayOfTsIter<'_, T> {
        self.into_iter()
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

impl<T> IntoIterator for SingleOrArray<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SingleOrArray::Single(s) => vec![s].into_iter(),
            SingleOrArray::Array(a) => a.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a SingleOrArray<T> {
    type IntoIter = TOrArrayOfTsIter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        TOrArrayOfTsIter {
            inner: self,
            idx: 0,
        }
    }
}

pub struct TOrArrayOfTsIter<'a, T> {
    inner: &'a SingleOrArray<T>,
    idx: usize,
}

impl<'a, T> Iterator for TOrArrayOfTsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.inner.len() {
            None
        } else {
            let item = match self.inner {
                SingleOrArray::Single(s) => Some(s),
                SingleOrArray::Array(v) => v.get(self.idx),
            };

            self.idx += 1;

            item
        }
    }
}

pub struct TOrArrayOfTsIterator<T> {
    inner: SingleOrArray<T>,
    idx: usize,
}

impl<T> Iterator for TOrArrayOfTsIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.inner.len() {
            None
        } else {
            let mut item: T = unsafe { std::mem::zeroed() };

            match &mut self.inner {
                SingleOrArray::Single(s) => std::mem::swap(&mut item, s),
                SingleOrArray::Array(v) => {
                    let found_item = unsafe { v.get_mut(self.idx).unwrap_unchecked() };
                    std::mem::swap(&mut item, found_item);
                }
            };

            self.idx += 1;

            Some(item)
        }
    }
}
