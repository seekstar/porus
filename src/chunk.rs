use crate::allocator::{allocate, deallocate, grow_to};
use crate::capacity::{DefaultPolicy, Policy};
use crate::pool::Pool;
use alloc::alloc::{Allocator, Global};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;

#[derive(Clone, Copy)]
#[rustc_layout_scalar_valid_range_start(0)]
#[cfg_attr(
    target_pointer_width = "64",
    rustc_layout_scalar_valid_range_end(0xFFFFFFFFFFFFFFFE)
)]
#[cfg_attr(
    target_pointer_width = "32",
    rustc_layout_scalar_valid_range_end(0xFFFFFFFE)
)]
pub struct Handle(usize);

#[repr(C)]
union Node<T> {
    data: ManuallyDrop<T>,
    next: Option<Handle>,
}

pub struct Chunk<T, P: Policy = DefaultPolicy, A: Allocator = Global> {
    size: usize,
    next: Option<Handle>,
    data: NonNull<[Node<T>]>,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: Policy, A: Allocator> Drop for Chunk<T, P, A> {
    fn drop(&mut self) {
        deallocate(&self.allocator, self.data);
    }
}

impl<T, P: Policy, A: Allocator> Chunk<T, P, A> {
    pub fn with_capacity_in(capacity: usize, allocator: A) -> Self {
        Self {
            size: 0,
            next: None,
            data: allocate(&allocator, P::initial(capacity)),
            allocator,
            _policy: PhantomData,
        }
    }

    pub fn new_in(allocator: A) -> Self {
        Self::with_capacity_in(0, allocator)
    }
}

impl<T, P: Policy, A: Allocator + Default> Chunk<T, P, A> {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, Default::default())
    }

    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
}

impl<T, P: Policy, A: Allocator + Default> Default for Chunk<T, P, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, P: Policy, A: Allocator> Pool<T> for Chunk<T, P, A> {
    type Handle = Handle;

    fn get(&self, handle: Handle) -> &T {
        unsafe { &*self.data.get_unchecked_mut(handle.0).as_ref().data }
    }

    fn get_mut(&mut self, handle: Handle) -> &mut T {
        unsafe { &mut *self.data.get_unchecked_mut(handle.0).as_mut().data }
    }

    fn add(&mut self, item: T) -> Handle {
        let index = match self.next {
            None => {
                let size = self.size;
                self.size = usize::wrapping_add(self.size, 1);
                if size == self.data.len() {
                    self.data = grow_to(&self.allocator, self.data, P::grow(size));
                }
                size
            }
            Some(handle) => {
                self.next = unsafe { self.data.get_unchecked_mut(handle.0).as_ref().next };
                handle.0
            }
        };

        unsafe { self.data.as_uninit_slice_mut().get_unchecked_mut(index) }.write(Node {
            data: ManuallyDrop::new(item),
        });

        unsafe { Handle(index) }
    }

    fn take(&mut self, handle: Handle) -> T {
        let index = handle.0;
        let p = unsafe { self.data.as_uninit_slice_mut().get_unchecked_mut(index) };
        let node = unsafe { p.assume_init_read() };
        p.write(Node { next: self.next });
        self.next = Some(handle);
        ManuallyDrop::into_inner(unsafe { node.data })
    }
}
