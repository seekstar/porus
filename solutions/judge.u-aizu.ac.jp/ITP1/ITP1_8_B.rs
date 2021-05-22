#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let mut sum: u64 = 0;
        let mut c: u8 = 0;
        read(Whitespace);
        while read(Char(&mut c)) && (c >= b'0') && (c <= b'9') {
            sum += (c - b'0') as u64;
        }

        if sum == 0 {
            break;
        }

        printf!("{sum:u}\n");
    }
}
