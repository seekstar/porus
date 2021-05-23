use crate::fmt::Bytes;
use crate::libc::scanf;
use alloc::alloc::{Allocator, Global, Layout};
use core::cmp::Ordering;
use core::convert::TryInto;
use core::mem::{forget, size_of, ManuallyDrop, MaybeUninit};
use core::ops::Deref;
use core::ptr::{copy_nonoverlapping, null_mut, NonNull};
use core::slice::from_raw_parts;
use core::str;

#[cfg(target_endian = "little")]
pub struct Shared<A: Allocator> {
    counter: *mut usize,
    allocator: A,
    length: usize,
    s: *const u8,
}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
pub struct Inline<const N: usize> {
    length: u8,
    s: [u8; N],
}

#[cfg(target_endian = "big")]
pub struct Shared<A: Allocator> {
    s: *mut u8,
    length: usize,
    allocator: A,
    counter: *const usize,
}

#[cfg(all(target_endian = "big", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
pub struct Inline<const N: usize> {
    s: [u8; N],
    length: u8,
}

impl<A: Allocator> AsRef<[u8]> for Shared<A> {
    fn as_ref(&self) -> &[u8] {
        unsafe { from_raw_parts(self.s, self.length) }
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
            length: self.length,
            s: self.s,
        }
    }
}

impl<A: Allocator> Drop for Shared<A> {
    fn drop(&mut self) {
        if let Some(mut p) = NonNull::new(self.counter) {
            #[allow(clippy::option_if_let_else)]
            if let Some(c) = usize::checked_sub(unsafe { *p.as_ref() }, 1) {
                unsafe {
                    *p.as_mut() = c;
                }
            } else {
                unsafe {
                    Allocator::deallocate(
                        &self.allocator,
                        p.cast(),
                        Layout::array::<u8>(usize::wrapping_add(size_of::<usize>(), self.length))
                            .unwrap(),
                    );
                }
            }
        }
    }
}

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
                length: s.len(),
                s: s.as_ptr(),
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
    allocator: MaybeUninit<A>,
    capacity: usize,
}

impl<A: Allocator> Drop for StringBuffer<A> {
    fn drop(&mut self) {
        let allocator = unsafe { MaybeUninit::assume_init_read(&self.allocator) };
        unsafe {
            Allocator::deallocate(
                &allocator,
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
    pub fn new(capacity: usize) -> Self
    where
        A: Default,
    {
        Self::new_with_allocator(capacity, Default::default())
    }

    pub fn new_with_allocator(capacity: usize, allocator: A) -> Self {
        let counter = Allocator::allocate(
            &allocator,
            Layout::array::<u8>(usize::wrapping_add(size_of::<usize>(), capacity)).unwrap(),
        )
        .unwrap()
        .cast();
        Self {
            counter,
            capacity,
            allocator: MaybeUninit::new(allocator),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_string(self, length: usize) -> String<A> {
        let allocator = unsafe { MaybeUninit::assume_init_read(&self.allocator) };

        let s = unsafe { self.counter.as_ptr().add(1) }.cast();

        let r = if length < size_of::<Shared<A>>() {
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
            String {
                shared: ManuallyDrop::new(Shared {
                    counter: self.counter.as_ptr(),
                    allocator,
                    length,
                    s,
                }),
            }
        };

        #[allow(clippy::mem_forget)]
        forget(self);
        r
    }

    pub fn scan(self) -> String<A> {
        let s = unsafe { self.counter.as_ptr().add(1) }.cast::<u8>();
        let (mut start, mut end): (i32, i32) = Default::default();
        unsafe { scanf(b" %n%s%n\0".as_ptr(), &mut start, s, &mut end) };
        self.to_string(i32::wrapping_sub(end, start).try_into().unwrap())
    }
}
