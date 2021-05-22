#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: u64 = read!();
    let b: u64 = read!();
    let c: u64 = read!();
    printf!(
        "{:u} {:u} {:u}",
        Ord::min(Ord::min(a, b), c),
        Ord::max(Ord::max(Ord::min(a, b), Ord::min(b, c)), Ord::min(a, c)),
        Ord::max(Ord::max(a, b), c)
    );
}
