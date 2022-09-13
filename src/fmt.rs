#[allow(clippy::integer_arithmetic)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::indexing_slicing)]
#[must_use]
pub const fn concat<const M: usize, const N: usize>(a: [u8; M], b: [u8; N]) -> [u8; M + N] {
    let mut c: [u8; M + N] = [0; M + N];
    let mut i = 0;
    while i < M {
        c[i] = a[i];
        i += 1;
    }
    let mut j = 0;
    while j < N {
        c[M + j] = b[j];
        j += 1;
    }
    c
}

pub trait Bytes {
    fn len(&self) -> usize;
    fn as_ptr(&self) -> *const u8;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Bytes for str {
    fn len(&self) -> usize {
        self.len()
    }

    fn as_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}

impl Bytes for [u8] {
    fn len(&self) -> usize {
        self.len()
    }

    fn as_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}

pub trait BytesMut {
    fn as_mut_ptr(&mut self) -> *mut u8;
}

impl BytesMut for [u8] {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
}

#[cfg(not(feature = "local-judge"))]
pub const FLOAT_ESCAPE_PREFIX: [u8; 0] = [];

#[cfg(not(feature = "local-judge"))]
pub const FLOAT_ESCAPE_SUFFIX: [u8; 0] = [];

#[cfg(feature = "local-judge")]
pub const FLOAT_ESCAPE_PREFIX: [u8; 4] = *b"\x1bXf.";

#[cfg(feature = "local-judge")]
pub const FLOAT_ESCAPE_SUFFIX: [u8; 2] = *b"\x1b\\";

#[cfg(windows)]
pub const PRI: [u8; 3] = *b"I64";
#[cfg(windows)]
pub const PRI64: [u8; 3] = *b"I64";
#[cfg(not(windows))]
pub const PRI: [u8; 2] = *b"ll";
#[cfg(not(windows))]
pub const PRI64: [u8; 2] = *b"ll";
pub const PRI32: [u8; 0] = *b"";
pub const PRI16: [u8; 1] = *b"h";
pub const PRI8: [u8; 2] = *b"hh";
pub const PRISIZE: [u8; 1] = *b"z";

pub fn interleave<T: Clone, F: FnMut(T), I: Iterator<Item = T>, S: FnMut()>(
    it: I,
    mut sep: S,
    mut f: F,
) {
    it.map(Some).intersperse(None).for_each(|e| e.map_or_else(|| {
        sep();
    }, |x| {
        f(x);
    }));
}
