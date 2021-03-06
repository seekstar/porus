pub trait Handle: Copy {}

pub trait Pool<T> {
    type Handle: Handle;

    fn get(&self, handle: Self::Handle) -> &T;
    fn get_mut(&mut self, handle: Self::Handle) -> &mut T;
    fn add(&mut self, item: T) -> Self::Handle;
    fn remove(&mut self, handle: Self::Handle) -> T;
}

pub fn get<T, P: Pool<T>>(pool: &P, handle: P::Handle) -> &T {
    Pool::get(pool, handle)
}

pub fn get_mut<T, P: Pool<T>>(pool: &mut P, handle: P::Handle) -> &mut T {
    Pool::get_mut(pool, handle)
}

pub fn add<T, P: Pool<T>>(pool: &mut P, item: T) -> P::Handle {
    Pool::add(pool, item)
}

pub fn remove<T, P: Pool<T>>(pool: &mut P, handle: P::Handle) -> T {
    Pool::remove(pool, handle)
}

use alloc::alloc::{Allocator, Layout};
use core::ptr::{read, write, NonNull};

#[derive(Clone, Copy)]
pub struct AllocHandle(NonNull<u8>);

impl Handle for AllocHandle {}

impl<T, A: Allocator> Pool<T> for A {
    type Handle = AllocHandle;

    fn get(&self, handle: Self::Handle) -> &T {
        unsafe { &*handle.0.cast().as_ptr() }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> &mut T {
        unsafe { &mut *handle.0.cast().as_ptr() }
    }

    fn add(&mut self, item: T) -> Self::Handle {
        unsafe {
            let mem = Allocator::allocate(self, Layout::new::<T>()).unwrap();
            write(mem.as_ptr().cast(), item);
            AllocHandle(mem.cast())
        }
    }

    fn remove(&mut self, handle: Self::Handle) -> T {
        unsafe {
            let item = read(handle.0.cast().as_ptr());
            Allocator::deallocate(self, handle.0, Layout::new::<T>());
            item
        }
    }
}
