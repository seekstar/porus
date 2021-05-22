#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: u64 = read!();
    let b: u64 = read!();
    let area = a * b;
    let perimeter = (a + b) * 2;
    printf!("{area:u} {perimeter:u}\n");
}
