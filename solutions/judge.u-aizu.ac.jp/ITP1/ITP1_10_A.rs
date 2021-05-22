#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let x1: f64 = read!();
    let y1: f64 = read!();
    let x2: f64 = read!();
    let y2: f64 = read!();
    printf!(
        "{:.5f}\n",
        sqrt((y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1))
    );
}
