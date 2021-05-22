#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: u64 = read!();
    let b: u64 = read!();
    let d = a / b;
    let r = a % b;
    let f = (a as f64) / (b as f64);
    printf!("{d:u} {r:u} {f:.6f}\n");
}
