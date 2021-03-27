pub trait Collection {
    fn size(&self) -> usize;
}

pub fn size<T: Collection>(c: &T) -> usize {
    Collection::size(c)
}

impl<T, const N: usize> Collection for [T; N] {
    fn size(&self) -> usize {
        N
    }
}

use alloc::vec::Vec;

impl<T> Collection for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

use alloc::collections::VecDeque;

impl<T> Collection for VecDeque<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

use alloc::collections::BinaryHeap;

impl<T: Ord> Collection for BinaryHeap<T> {
    fn size(&self) -> usize {
        self.len()
    }
}
