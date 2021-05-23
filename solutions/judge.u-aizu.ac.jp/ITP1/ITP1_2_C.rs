#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let (mut a, mut b, mut c) = default();
    scanf!("{:u} {:u} {:u}", &mut a, &mut b, &mut c);
    printf!(
        "{:u} {:u} {:u}",
        Ord::min(Ord::min(a, b), c),
        Ord::max(Ord::max(Ord::min(a, b), Ord::min(b, c)), Ord::min(a, c)),
        Ord::max(Ord::max(a, b), c)
    );
}
