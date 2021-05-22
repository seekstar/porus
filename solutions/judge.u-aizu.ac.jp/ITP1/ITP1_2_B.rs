#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: u64 = read!();
    let b: u64 = read!();
    let c: u64 = read!();
    printf!("{}", if (a < b) && (b < c) { "Yes" } else { "No" });
}
