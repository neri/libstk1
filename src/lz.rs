//! Lempel-Ziv compression utilities

#[inline]
#[track_caller]
pub(super) fn matching_len<T>(data: &[T], current: usize, distance: usize, max_len: usize) -> usize
where
    T: Copy + PartialEq,
{
    assert!(
        data.len() > current && distance != 0 && current >= distance,
        "INVALID MATCHES: LEN {} CURRENT {} DISTANCE {}",
        data.len(),
        current,
        distance
    );
    unsafe {
        let max_len = (data.len() - current).min(max_len);
        let mut p = data.as_ptr().add(current);
        let mut q = data.as_ptr().add(current - distance);
        for len in 0..max_len {
            if p.read_volatile() != q.read_volatile() {
                return len;
            }
            p = p.add(1);
            q = q.add(1);
        }
        max_len
    }
}

/// Matching distance and length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matches {
    pub len: usize,
    pub distance: usize,
}

impl Matches {
    pub const ZERO: Self = Self {
        len: 0,
        distance: 0,
    };

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.len == 0
    }
}

impl Default for Matches {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
