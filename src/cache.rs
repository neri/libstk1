//! cache offsets of matching patterns

use alloc::{boxed::Box, collections::BTreeMap};
use core::mem::ManuallyDrop;

pub type OffsetCache<'a> = MatchingCache<'a, MatchingBytesKey>;

pub struct MatchingCache<'a, KEY>
where
    KEY: MatchingKey,
{
    input: &'a [KEY::ElementType],
    key: KEY,
    cache: BTreeMap<KEY::KeyType, OffsetList>,
    cursor: usize,
    limit: usize,
    max_distance: usize,
}

impl<'a, KEY: MatchingKey> MatchingCache<'a, KEY> {
    #[inline]
    pub fn new(input: &'a [KEY::ElementType], max_distance: usize) -> Self {
        if input.len() < 4 {
            Self {
                input,
                key: KEY::null(),
                cache: BTreeMap::new(),
                cursor: 0,
                limit: 0,
                max_distance,
            }
        } else {
            Self {
                input,
                key: KEY::new(input[0], input[1], input[2]),
                cache: BTreeMap::new(),
                cursor: 0,
                limit: input.len() - 2,
                max_distance,
            }
        }
    }
}

impl<KEY: MatchingKey> MatchingCache<'_, KEY> {
    pub fn advance(&mut self, step: usize) {
        let limit = self.limit;
        let mut cursor = self.cursor;
        if cursor >= limit {
            return;
        }
        for _ in 0..step {
            self._insert(self.key.key_value(), cursor);
            cursor += 1;
            if cursor >= limit {
                break;
            }
            self.key.advance(self.input[cursor + 2]);
        }
        self.cursor = cursor;
    }

    pub fn matches<'a>(&'a self) -> Option<impl Iterator<Item = usize> + 'a> {
        if self.cursor >= self.limit {
            return None;
        }
        let min_value = self.cursor.saturating_sub(self.max_distance);
        self.cache
            .get(&self.key.key_value())
            .map(|v| v.distance_iter(self.cursor, min_value))
    }

    fn _insert(&mut self, key: KEY::KeyType, value: usize) {
        let value = value as u32;
        match self.cache.get_mut(&key) {
            Some(list) => {
                list.push(value);
            }
            None => {
                self.cache.insert(key, OffsetList::new(value));
            }
        }

        if self.cache.len() >= self.max_distance * 2 {
            let min_value = self.cursor.saturating_sub(self.max_distance) as u32;
            self.cache.retain(|_k, v| v.retain(min_value))
        }
    }
}

pub trait MatchingKey
where
    Self::ElementType: Copy,
    Self::KeyType: Copy + Ord,
{
    type ElementType;
    type KeyType;

    fn null() -> Self;

    fn new(val0: Self::ElementType, val1: Self::ElementType, val2: Self::ElementType) -> Self;

    fn key_value(&self) -> Self::KeyType;

    fn advance(&mut self, new_value: Self::ElementType);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatchingBytesKey(u32);

impl MatchingKey for MatchingBytesKey {
    type ElementType = u8;
    type KeyType = u32;

    #[inline]
    fn null() -> Self {
        Self(Default::default())
    }

    #[inline]
    fn new(val0: Self::ElementType, val1: Self::ElementType, val2: Self::ElementType) -> Self {
        Self(((val0 as u32) << 16) | ((val1 as u32) << 8) | (val2 as u32))
    }

    #[inline]
    fn key_value(&self) -> Self::KeyType {
        self.0
    }

    #[inline]
    fn advance(&mut self, new_value: Self::ElementType) {
        self.0 = ((self.0 << 8) | (new_value as u32)) & 0xFF_FF_FF;
    }
}

pub struct OffsetList {
    root: ManuallyDrop<Box<SimpleList<u32>>>,
}

#[allow(dead_code)]
impl OffsetList {
    #[inline]
    pub fn new(value: u32) -> Self {
        Self {
            root: ManuallyDrop::new(SimpleList::new_leaf(value)),
        }
    }

    pub fn push(&mut self, value: u32) {
        unsafe {
            let next = ManuallyDrop::take(&mut self.root);
            self.root = ManuallyDrop::new(SimpleList::new(value, next));
        }
    }

    #[inline]
    pub fn root<'a>(&'a self) -> &'a Box<SimpleList<u32>> {
        &self.root
    }

    #[inline]
    #[track_caller]
    pub fn nearest(&self) -> u32 {
        self.root.value()
    }

    pub fn retain(&mut self, min_value: u32) -> bool {
        if self.nearest() < min_value {
            return false;
        }
        let mut node = (&mut self.root) as &mut Box<SimpleList<u32>>;
        loop {
            if let Some(next) = node.next() {
                if next.value() < min_value {
                    node.unlink();
                    break;
                }
            }
            match node.next_mut() {
                Some(next) => {
                    node = next;
                }
                None => break,
            }
        }
        true
    }

    #[inline]
    pub fn distance_iter<'a>(
        &'a self,
        current: usize,
        min_value: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        DistanceIter {
            node: Some(&self.root),
            current,
            min_value,
        }
    }
}

impl Drop for OffsetList {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.root);
        }
    }
}

pub struct SimpleList<T> {
    next: Option<Box<Self>>,
    value: T,
}

#[allow(dead_code)]
impl<T> SimpleList<T> {
    #[inline]
    pub fn new(value: T, next: Box<Self>) -> Box<Self> {
        Box::new(Self {
            value,
            next: Some(next),
        })
    }

    #[inline]
    pub fn new_leaf(value: T) -> Box<Self> {
        Box::new(Self { value, next: None })
    }

    #[inline]
    pub fn link(&mut self, next: Box<Self>) {
        self.next = Some(next);
    }

    #[inline]
    pub fn unlink(&mut self) {
        let next = self.next.take();
        drop(next);
    }

    #[inline]
    pub fn next(&self) -> Option<&Box<Self>> {
        self.next.as_ref()
    }

    #[inline]
    pub fn next_mut(&mut self) -> Option<&mut Box<Self>> {
        self.next.as_mut()
    }

    #[inline]
    pub fn as_ref(&self) -> &T {
        &self.value
    }

    #[inline]
    pub fn value(&self) -> T
    where
        T: Copy,
    {
        self.value
    }
}

impl<T> Drop for SimpleList<T> {
    fn drop(&mut self) {
        let mut next = self.next.take();
        loop {
            match next {
                Some(mut v) => next = v.next.take(),
                None => break,
            }
        }
    }
}

struct DistanceIter<'a> {
    node: Option<&'a Box<SimpleList<u32>>>,
    current: usize,
    min_value: usize,
}

impl Iterator for DistanceIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            self.node = node.next();
            if (node.value() as usize) >= self.min_value {
                return Some(self.current - node.value() as usize);
            }
        }
        None
    }
}
