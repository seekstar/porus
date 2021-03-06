use crate::libc;
use alloc::alloc::{GlobalAlloc, Layout};
use core::cmp::min;
use core::ptr::{copy_nonoverlapping, null_mut};

#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone)]
pub struct System;

// libstd/sys_common/alloc.rs
#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
pub const MIN_ALIGN: usize = 8;
#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
pub const MIN_ALIGN: usize = 16;

#[allow(clippy::trivially_copy_pass_by_ref)]
pub unsafe fn realloc_fallback(
    alloc: &System,
    ptr: *mut u8,
    old_layout: Layout,
    new_size: usize,
) -> *mut u8 {
    // Docs for GlobalAlloc::realloc require this to be valid:
    let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

    let new_ptr = GlobalAlloc::alloc(alloc, new_layout);
    if !new_ptr.is_null() {
        let size = min(old_layout.size(), new_size);
        copy_nonoverlapping(ptr, new_ptr, size);
        GlobalAlloc::dealloc(alloc, ptr, old_layout);
    }
    new_ptr
}

// libstd/sys/unix/alloc.rs
#[cfg(unix)]
unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
    let mut out = null_mut();
    let ret = libc::posix_memalign(&mut out, layout.align(), layout.size());
    if ret == 0_i32 {
        out
    } else {
        null_mut()
    }
}

#[cfg(windows)]
unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
    libc::_aligned_malloc(layout.size(), layout.align())
}

unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            libc::malloc(layout.size())
        } else {
            aligned_malloc(&layout)
        }
    }

    #[cfg(unix)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr);
    }

    #[cfg(windows)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            libc::free(ptr)
        } else {
            libc::_aligned_free(ptr)
        }
    }

    #[cfg(unix)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
            libc::realloc(ptr, new_size)
        } else {
            realloc_fallback(self, ptr, layout, new_size)
        }
    }

    #[cfg(windows)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
            libc::realloc(ptr, new_size)
        } else {
            libc::_aligned_realloc(ptr, new_size, layout.align())
        }
    }
}
