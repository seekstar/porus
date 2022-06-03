use crate::fmt::Bytes;
use crate::libc::scanf;
use alloc::alloc::{Allocator, Global, Layout};
use core::cmp::Ordering;
use core::convert::TryInto;
use core::mem::{forget, size_of, transmute_copy, ManuallyDrop};
use core::ops::Deref;
use core::ptr::{copy_nonoverlapping, null_mut, NonNull};
use core::str;

#[cfg(target_endian = "little")]
pub struct Shared<A: Allocator> {
    counter: *mut usize,
    allocator: A,
    s: NonNull<[u8]>,
}

#[cfg(all(target_endian = "little"))]
#[derive(Clone, Copy)]
pub struct Inline<const N: usize> {
    length: u8,
    s: [u8; N],
}

#[cfg(target_endian = "big")]
pub struct Shared<A: Allocator> {
    s: NonNull<[u8]>,
    allocator: A,
    counter: *const usize,
}

#[cfg(all(target_endian = "big"))]
#[derive(Clone, Copy)]
pub struct Inline<const N: usize> {
    s: [u8; N],
    length: u8,
}

impl<A: Allocator> AsRef<[u8]> for Shared<A> {
    fn as_ref(&self) -> &[u8] {
        unsafe { self.s.as_ref() }
    }
}

impl<const N: usize> AsRef<[u8]> for Inline<N> {
    fn as_ref(&self) -> &[u8] {
        self.s
            .as_ref()
            .get(..Into::into(u8::wrapping_shr(self.length, 1)))
            .unwrap()
    }
}

impl<A: Allocator + Clone> Clone for Shared<A> {
    fn clone(&self) -> Self {
        if let Some(mut p) = NonNull::new(self.counter) {
            unsafe {
                *p.as_mut() = usize::wrapping_add(*p.as_ref(), 1);
            }
        }

        Self {
            counter: self.counter,
            allocator: Clone::clone(&self.allocator),
            s: self.s,
        }
    }
}

impl<A: Allocator> Drop for Shared<A> {
    fn drop(&mut self) {
        if let Some(mut p) = NonNull::new(self.counter) {
            match usize::checked_sub(unsafe { *p.as_ref() }, 1) {
                None => unsafe {
                    Allocator::deallocate(
                        &self.allocator,
                        p.cast(),
                        Layout::array::<u8>(usize::wrapping_add(size_of::<usize>(), self.s.len()))
                            .unwrap(),
                    );
                },
                Some(c) => unsafe {
                    *p.as_mut() = c;
                },
            }
        }
    }
}

#[repr(C)]
pub union String<A: Allocator = Global>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    shared: ManuallyDrop<Shared<A>>,
    inline: Inline<{ size_of::<Shared<A>>() - 1 }>,
}

impl<A: Allocator> String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    fn is_inline(&self) -> bool {
        (unsafe { &self.inline }.length & 1) == 1
    }
}

impl<A: Allocator> Deref for String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_ref()) }
    }
}

impl<A: Allocator> AsRef<[u8]> for String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    fn as_ref(&self) -> &[u8] {
        if self.is_inline() {
            unsafe { &self.inline }.as_ref()
        } else {
            unsafe { &*self.shared }.as_ref()
        }
    }
}

impl<A: Allocator> PartialEq for String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.as_ref(), other.as_ref())
    }
}

impl<A: Allocator> PartialOrd for String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.as_ref(), other.as_ref())
    }
}

impl<A: Allocator + Default> From<&'static [u8]> for String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    fn from(s: &'static [u8]) -> Self {
        Self {
            shared: ManuallyDrop::new(Shared {
                counter: null_mut(),
                allocator: Default::default(),
                s: From::from(s),
            }),
        }
    }
}

impl<A: Allocator + Clone> Clone for String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    fn clone(&self) -> Self {
        if self.is_inline() {
            Self {
                inline: Clone::clone(unsafe { &self.inline }),
            }
        } else {
            Self {
                shared: Clone::clone(unsafe { &self.shared }),
            }
        }
    }
}

impl<A: Allocator> Bytes for String<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn as_ptr(&self) -> *const u8 {
        self.as_ref().as_ptr()
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct StringBuffer<A: Allocator = Global> {
    counter: NonNull<usize>,
    allocator: A,
    capacity: usize,
}

impl<A: Allocator> Drop for StringBuffer<A> {
    fn drop(&mut self) {
        unsafe {
            Allocator::deallocate(
                &self.allocator,
                self.counter.cast(),
                Layout::array::<u8>(usize::wrapping_add(size_of::<usize>(), self.capacity))
                    .unwrap(),
            );
        }
    }
}

impl<A: Allocator> StringBuffer<A>
where
    Inline<{ size_of::<Shared<A>>() - 1 }>: Sized,
{
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self
    where
        A: Default,
    {
        Self::with_capacity_in(capacity, Default::default())
    }

    pub fn with_capacity_in(capacity: usize, allocator: A) -> Self {
        let counter = Allocator::allocate(
            &allocator,
            Layout::array::<u8>(usize::wrapping_add(size_of::<usize>(), capacity)).unwrap(),
        )
        .unwrap()
        .cast();
        Self {
            counter,
            allocator,
            capacity,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_string(self, length: usize) -> String<A> {
        let counter = self.counter;
        let s = unsafe { counter.as_ptr().add(1) }.cast();

        if length < size_of::<Shared<A>>() {
            #[allow(clippy::cast_possible_truncation)]
            let mut i = Inline {
                length: u8::wrapping_shl(length as u8, 1) | 1,
                s: [0; size_of::<Shared<A>>() - 1],
            };

            unsafe {
                copy_nonoverlapping(s, i.s.as_mut_slice().as_mut_ptr(), length);
            }

            String { inline: i }
        } else {
            let allocator: A = unsafe { transmute_copy(&self.allocator) };
            let capacity = self.capacity;
            #[allow(clippy::mem_forget)]
            {
                forget(self);
            }

            let block = unsafe {
                Allocator::shrink(
                    &allocator,
                    counter.cast(),
                    Layout::array::<u8>(usize::wrapping_add(size_of::<usize>(), capacity)).unwrap(),
                    Layout::array::<u8>(usize::wrapping_add(size_of::<usize>(), length)).unwrap(),
                )
            }
            .unwrap();

            String {
                shared: ManuallyDrop::new(Shared {
                    counter: counter.as_ptr(),
                    allocator,
                    s: NonNull::slice_from_raw_parts(block.cast(), length),
                }),
            }
        }
    }

    pub fn scan(self) -> String<A> {
        let s = unsafe { self.counter.as_ptr().add(1) }.cast::<u8>();
        let (mut start, mut end): (i32, i32) = Default::default();
        unsafe { scanf(b" %n%s%n\0".as_ptr(), &mut start, s, &mut end) };
        self.to_string(i32::wrapping_sub(end, start).try_into().unwrap())
    }
}
