#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a = &mut Vec::new();
    let mut s = [0u8; 7];

    while scanf!(" {:s}", s.as_mut_slice()) {
        match s[0] {
            b'+' => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x + y);
            }
            b'-' => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x - y);
            }
            b'*' => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x * y);
            }
            _ => {
                let mut x = default();
                sscanf!(s.as_slice(), "{:i}", &mut x);
                stack::push(a, x);
            }
        }
    }

    printf!("{:i}\n", stack::pop(a));
}
