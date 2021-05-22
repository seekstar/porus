#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let x: u64 = read!();
    let cube = x * x * x;
    printf!("{cube:u}\n");
}
