#[allow(unused_imports)]
use crate::io::{PeekableSource, Sink, Source};
use crate::scan::{fread, Consumer, Whitespace};
use crate::utils::unwrap;
use core::cell::Cell;
use core::iter::Iterator;
use core::ptr::NonNull;

pub struct Input(Cell<Option<NonNull<dyn Source<Item = u8>>>>);
pub struct Output(Cell<Option<NonNull<dyn Sink>>>);

impl Iterator for Input {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        Iterator::next(unsafe { unwrap(self.0.get()).as_mut() })
    }
}

impl Sink for Output {
    fn write(&mut self, c: u8) {
        Sink::write(unsafe { unwrap(self.0.get()).as_mut() }, c);
    }
}

static mut STDIN: PeekableSource<Input> = PeekableSource::new(Input(Cell::new(None)));
static mut STDOUT: Output = Output(Cell::new(None));

#[allow(clippy::option_option)]
struct PeekableSourceInput {
    source: Input,
    _peeked: Option<Option<u8>>,
}

#[allow(clippy::ptr_as_ptr)]
pub fn initialize(stdin: *mut dyn Source<Item = u8>, stdout: *mut dyn Sink) {
    unsafe {
        (*(&STDIN as *const _ as *const PeekableSourceInput))
            .source
            .0
            .set(NonNull::new(stdin));
        STDOUT.0.set(NonNull::new(stdout));
    }
}

pub fn read<C: Consumer>(c: C) -> bool {
    unsafe { fread(&mut STDIN, c) }
}

pub fn read_skip_ws<C: Consumer>(c: C) -> bool {
    read(Whitespace);
    read(c)
}

pub macro read_opt() {{
    let mut x = Default::default();
    if read_skip_ws(&mut x) {
        Some(x)
    } else {
        None
    }
}}

pub macro read {
    () => {
        {
            read_opt!().unwrap()
        }
    },
    ( $($expr:expr),* ) => {
        $(
            read_skip_ws($expr);
        )*
    }
}
