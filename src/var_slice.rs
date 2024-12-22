pub struct VarSlice<'a, T> {
    source: &'a [T],
    offset: usize,
    len: usize,
}

impl<'a, T> VarSlice<'a, T> {
    #[inline]
    pub fn new(source: &'a [T], offset: usize) -> Self {
        Self {
            source,
            offset,
            len: 1,
        }
    }

    #[inline]
    pub fn into_slice(self) -> &'a [T] {
        &self.source[self.offset..self.offset + self.len]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> VarSlice<'_, T> {
    #[inline]
    pub fn expand(&mut self, delta: usize) {
        self.len += delta;
    }
}
