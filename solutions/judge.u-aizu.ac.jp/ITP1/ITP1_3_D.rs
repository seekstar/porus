#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: u64 = read!();
    let b: u64 = read!();
    let c: u64 = read!();
    printf!("{:usize}\n", (a..=b).filter(|x| (&c) % x == 0).count());
}
