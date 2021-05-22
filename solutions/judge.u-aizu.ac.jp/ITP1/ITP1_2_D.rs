#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let w: i64 = read!();
    let h: i64 = read!();
    let x: i64 = read!();
    let y: i64 = read!();
    let r: i64 = read!();
    printf!(
        "{}",
        if (r <= x) && (x <= (w - r)) && (r <= y) && (y <= (h - r)) {
            "Yes"
        } else {
            "No"
        }
    );
}
