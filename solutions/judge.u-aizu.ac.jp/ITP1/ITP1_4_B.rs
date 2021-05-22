#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let r: f64 = read!();
    let area = PI * r * r;
    let circumference = PI * 2.0 * r;
    printf!("{area:.6f} {circumference:.6f}\n");
}
