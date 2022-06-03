use crate::collection::Collection;
use crate::heap::Heap;
use crate::list::{List, ListMut};
use crate::stack::{self, Stack};

#[must_use]
pub fn parent_index(d: usize, index: usize) -> Option<usize> {
    usize::checked_sub(index, 1).map(|i| usize::wrapping_div(i, d))
}

#[must_use]
pub const fn child_index(d: usize, index: usize, n: usize) -> usize {
    usize::saturating_add(usize::saturating_mul(d, index), usize::saturating_add(1, n))
}

pub fn siftup<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(
    d: usize,
    l: &mut L,
    n: usize,
    gt: F,
) {
    if let Some(parent) = parent_index(d, n) {
        if !gt(List::get(l, n).unwrap(), List::get(l, parent).unwrap()) {
            return;
        }
        List::swap(l, n, parent);
        siftup(d, l, parent, gt);
    }
}

pub fn siftdown<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(
    d: usize,
    l: &mut L,
    n: usize,
    gt: F,
) {
    let largest = (child_index(d, n, 0)..Ord::min(Collection::size(l), child_index(d, n, d))).fold(
        n,
        |largest, c| {
            if gt(List::get(l, c).unwrap(), List::get(l, largest).unwrap()) {
                c
            } else {
                largest
            }
        },
    );

    if largest > n {
        List::swap(l, n, largest);
        siftdown(d, l, largest, gt);
    }
}

pub fn heapify<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(d: usize, l: &mut L, gt: F) {
    if let Some(index) = usize::checked_sub(Collection::size(l), 1) {
        if let Some(parent) = parent_index(d, index) {
            let mut n = parent;
            loop {
                siftdown(d, l, n, &gt);
                if let Some(n1) = usize::checked_sub(n, 1) {
                    n = n1;
                } else {
                    break;
                }
            }
        }
    }
}

pub struct DHeap<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool> {
    d: usize,
    list: L,
    gt: F,
}

impl<E, L: ListMut<Elem = E> + Stack<Elem = E>, F: Fn(&E, &E) -> bool> DHeap<E, L, F> {
    pub const fn new(d: usize, list: L, gt: F) -> Self {
        Self { d, list, gt }
    }
}

impl<E, L: ListMut<Elem = E> + Stack<Elem = E>, F: Fn(&E, &E) -> bool> Collection
    for DHeap<E, L, F>
{
    fn size(&self) -> usize {
        Collection::size(&self.list)
    }
}

impl<E, L: ListMut<Elem = E> + Stack<Elem = E>, F: Fn(&E, &E) -> bool> Heap for DHeap<E, L, F> {
    type Elem = E;

    fn push(&mut self, item: E) {
        let size = Collection::size(self);
        stack::push(&mut self.list, item);
        siftup(self.d, &mut self.list, size, &self.gt);
    }

    fn pop(&mut self) -> Option<E> {
        match usize::checked_sub(Collection::size(self), 1) {
            None => None,
            Some(index) => {
                List::swap(&mut self.list, 0, index);
                let result = stack::pop(&mut self.list);
                siftdown(self.d, &mut self.list, 0, &self.gt);
                Some(result)
            }
        }
    }

    fn peek(&self) -> Option<&E> {
        List::get(&self.list, 0)
    }
}
