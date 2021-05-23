use crate::block::Block;
use crate::capacity::{DefaultPolicy, Policy};
use crate::pool::Pool;
use alloc::alloc::{Allocator, Global};
use core::mem::ManuallyDrop;

#[derive(Clone, Copy)]
#[rustc_layout_scalar_valid_range_start(0)]
#[cfg_attr(target_pointer_width = "64", rustc_layout_scalar_valid_range_start(0xFFFFFFFFFFFFFFFE))]
#[cfg_attr(target_pointer_width = "32", rustc_layout_scalar_valid_range_end(0xFFFFFFFE))]
pub struct Handle(usize);


union Node<T> {
    data: ManuallyDrop<T>,
    next: Option<Handle>,
}

pub struct Chunk<T, P: Policy = DefaultPolicy, A: Allocator = Global> {
    size: usize,
    next: Option<Handle>,
    data: Block<Node<T>, P, A>,
}

impl<T, P: Policy, A: Allocator> Chunk<T, P, A> {
    pub fn new(allocator: A, capacity: usize) -> Self {
        Self {
            size: 0,
            next: None,
            data: Block::new(allocator, capacity),
        }
    }
}

impl<T, P: Policy, A: Allocator + Default> Chunk<T, P, A> {
    #[must_use]
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self::new(Default::default(), capacity)
    }
}

impl<T, P: Policy, A: Allocator> Pool<T> for Chunk<T, P, A> {
    type Handle = Handle;

    fn get(&self, handle: Handle) -> &T {
        unsafe { &self.data.get(handle.0).data }
    }

    fn get_mut(&mut self, handle: Handle) -> &mut T {
        unsafe { &mut self.data.get_mut(handle.0).data }
    }

    fn add(&mut self, item: T) -> Handle {
        let index = match self.next {
            None => {
                let size = self.size;
                self.size = usize::wrapping_add(self.size, 1);
                if size == self.data.capacity() {
                    assert!(self.data.grow(0) > 0);
                }
                size
            }
            Some(handle) => {
                self.next = unsafe { self.data.get(handle.0).next };
                handle.0
            }
        };

        self.data.write(
            index,
            Node {
                data: ManuallyDrop::new(item),
            },
        );
        unsafe {
            Handle(index)
        }
    }

    fn remove(&mut self, handle: Handle) -> T {
        let index = handle.0;
        let node = self.data.read(index);
        self.data.write(index, Node { next: self.next });
        self.next = Some(handle);
        ManuallyDrop::into_inner(unsafe { node.data })
    }
}
