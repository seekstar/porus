#[macro_use]
extern crate porus;
prelude!();

fn print(x: u32) {
    (0..32usize).for_each(|i| printf!("{:u}", ((x << i) >> 31) & 1));
    printf!("\n");
}

fn main() {
    let a: u32 = read!();
    let b: u32 = read!();
    print(a & b);
    print(a | b);
    print(a ^ b);
}
