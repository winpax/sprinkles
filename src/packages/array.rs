//! Array helpers (currently unused)

use super::models::manifest::TOrArrayOfTs;

impl<T> TOrArrayOfTs<T> {
    /// Get an iterator over the array
    pub fn iter(&self) -> TOrArrayOfTsIter<'_, T> {
        self.into_iter()
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        match self {
            TOrArrayOfTs::Single(_) => 1,
            TOrArrayOfTs::Array(a) => a.len(),
        }
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> IntoIterator for TOrArrayOfTs<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            TOrArrayOfTs::Single(s) => vec![s].into_iter(),
            TOrArrayOfTs::Array(a) => a.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a TOrArrayOfTs<T> {
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
    inner: &'a TOrArrayOfTs<T>,
    idx: usize,
}

impl<'a, T> Iterator for TOrArrayOfTsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.inner.len() {
            None
        } else {
            let item = match self.inner {
                TOrArrayOfTs::Single(s) => Some(s),
                TOrArrayOfTs::Array(v) => v.get(self.idx),
            };

            self.idx += 1;

            item
        }
    }
}

pub struct TOrArrayOfTsIterator<T> {
    inner: TOrArrayOfTs<T>,
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
                TOrArrayOfTs::Single(s) => std::mem::swap(&mut item, s),
                TOrArrayOfTs::Array(v) => {
                    let found_item = unsafe { v.get_mut(self.idx).unwrap_unchecked() };
                    std::mem::swap(&mut item, found_item);
                }
            };

            self.idx += 1;

            Some(item)
        }
    }
}
