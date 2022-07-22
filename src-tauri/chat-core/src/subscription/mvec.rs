use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct MVec<T> {
    counter: usize,
    inner: HashMap<usize, T>,
}

#[derive(Debug)]
pub(crate) struct MVecIterMut<'a, T> {
    inner: std::collections::hash_map::ValuesMut<'a, usize, T>,
}

impl<T> MVec<T> {
    pub(crate) fn counter(&self) -> usize {
        self.counter
    }

    pub(crate) fn push(&mut self, item: T) {
        self.inner.insert(self.counter, item);
        self.counter += 1;
    }

    pub(crate) fn remove(&mut self, index: usize) -> Option<T> {
        self.inner.remove(&index)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub(crate) fn iter_mut(&mut self) -> MVecIterMut<'_, T> {
        MVecIterMut {
            inner: self.inner.values_mut(),
        }
    }
}

impl<T> Default for MVec<T> {
    fn default() -> Self {
        Self {
            counter: Default::default(),
            inner: HashMap::default(),
        }
    }
}

impl<'a, T> Iterator for MVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
