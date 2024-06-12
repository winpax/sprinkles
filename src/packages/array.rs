use super::models::manifest::TOrArrayOfTs;

impl<T> TOrArrayOfTs<T> {
    pub fn iter(&self) -> TOrArrayOfTsIter<'_, T> {
        TOrArrayOfTsIter {
            inner: self,
            idx: 0,
        }
    }

    pub fn into_iter(self) -> TOrArrayOfTsIterator<T> {
        TOrArrayOfTsIterator {
            inner: self,
            idx: 0,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            TOrArrayOfTs::Single(_) => 1,
            TOrArrayOfTs::Array(a) => a.len(),
        }
    }
}

pub struct TOrArrayOfTsIter<'a, T: 'a> {
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
