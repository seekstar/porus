use core::alloc::{Allocator, Layout};
use core::ptr::NonNull;

pub fn allocate<A: Allocator, T>(allocator: &A, capacity: usize) -> NonNull<[T]> {
    let block = Allocator::allocate(allocator, Layout::array::<T>(capacity).unwrap()).unwrap();
    NonNull::slice_from_raw_parts(block.cast(), capacity)
}

pub fn grow_to<A: Allocator, T>(
    allocator: &A,
    block: NonNull<[T]>,
    capacity: usize,
) -> NonNull<[T]> {
    let new_block = unsafe {
        Allocator::grow(
            allocator,
            block.cast(),
            Layout::array::<T>(block.len()).unwrap(),
            Layout::array::<T>(capacity).unwrap(),
        )
    }
    .unwrap();
    NonNull::slice_from_raw_parts(new_block.cast(), capacity)
}

pub fn shrink_to<A: Allocator, T>(
    allocator: &A,
    block: NonNull<[T]>,
    capacity: usize,
) -> NonNull<[T]> {
    let new_block = unsafe {
        Allocator::shrink(
            allocator,
            block.cast(),
            Layout::array::<T>(block.len()).unwrap(),
            Layout::array::<T>(capacity).unwrap(),
        )
    }
    .unwrap();
    NonNull::slice_from_raw_parts(new_block.cast(), capacity)
}

pub fn deallocate<A: Allocator, T>(allocator: &A, block: NonNull<[T]>) {
    unsafe {
        Allocator::deallocate(
            allocator,
            block.cast(),
            Layout::array::<T>(block.len()).unwrap(),
        );
    }
}
