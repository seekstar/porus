#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: f64 = read!();
    let b: f64 = read!();
    let c: f64 = read!();

    let d = f64::to_radians(c);
    let h = b * sin(d);
    let w = b * cos(d);

    printf!("{:.5f}\n", a * h / 2.0);
    printf!("{:.5f}\n", (a + b + sqrt(h * h + (a - w) * (a - w))));
    printf!("{:.5f}\n", h);
}
