#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: i64 = read!();
    let b: i64 = read!();
    let c: i64 = read!();
    printf!(
        "{:i} {:i}\n",
        Ord::min(Ord::min(a, b), c),
        Ord::max(Ord::max(a, b), c)
    );
}
