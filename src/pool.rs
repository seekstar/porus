use core::alloc::{Allocator, Layout};
use core::ptr::{read, write, NonNull};

pub trait Pool<T> {
    type Handle: Copy;

    fn get(&self, handle: Self::Handle) -> &T;
    fn get_mut(&mut self, handle: Self::Handle) -> &mut T;
    fn add(&mut self, item: T) -> Self::Handle;
    fn take(&mut self, handle: Self::Handle) -> T;
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

pub fn take<T, P: Pool<T>>(pool: &mut P, handle: P::Handle) -> T {
    Pool::take(pool, handle)
}

impl<T, A: Allocator> Pool<T> for A {
    type Handle = NonNull<u8>;

    fn get(&self, handle: Self::Handle) -> &T {
        unsafe { handle.cast().as_ref() }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> &mut T {
        unsafe { handle.cast().as_mut() }
    }

    fn add(&mut self, item: T) -> Self::Handle {
        unsafe {
            let mem = Allocator::allocate(self, Layout::new::<T>()).unwrap();
            write(mem.cast().as_ptr(), item);
            mem.cast()
        }
    }

    fn take(&mut self, handle: Self::Handle) -> T {
        unsafe {
            let item = read(handle.cast().as_ptr());
            Allocator::deallocate(self, handle.cast(), Layout::new::<T>());
            item
        }
    }
}
