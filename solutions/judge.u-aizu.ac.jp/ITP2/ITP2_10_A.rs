#[macro_use]
extern crate porus;
prelude!();

fn print(x: u32) {
    (0..32usize).for_each(|i| printf!("{:u}", ((x << i) >> 31) & 1));
    printf!("\n");
}

fn main() {
    let x: u32 = read!();
    print(x);
    print(!x);
    print(x << 1);
    print(x >> 1);
}
