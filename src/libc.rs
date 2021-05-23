use core::fmt;

#[derive(Debug)]
pub struct Error(i32);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({})", self.0)
    }
}

extern "C" {
    pub fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
    pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;

    pub fn printf(fmt: *const u8, ...) -> i32;
    pub fn scanf(fmt: *const u8, ...) -> i32;
    pub fn sscanf(s: *const u8, fmt: *const u8, ...) -> i32;

    #[cfg(unix)]
    pub fn posix_memalign(memptr: *mut *mut u8, alignment: usize, size: usize) -> i32;
    #[cfg(windows)]
    pub fn _aligned_malloc(size: usize, alignment: usize) -> *mut u8;
    #[cfg(windows)]
    pub fn _aligned_realloc(ptr: *mut u8, size: usize, alignment: usize) -> *mut u8;
    #[cfg(windows)]
    pub fn _aligned_free(ptr: *mut u8);

    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
    pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8;

    #[cfg_attr(target_os = "windows", link_name = "_errno")]
    #[cfg_attr(target_os = "macos", link_name = "__error")]
    #[cfg_attr(target_os = "linux", link_name = "__errno_location")]
    fn errno_location() -> *mut i32;
}

fn get_errno() -> i32 {
    unsafe { *errno_location() }
}

pub fn get_error<T>() -> Result<T, Error> {
    Err(Error(get_errno()))
}
