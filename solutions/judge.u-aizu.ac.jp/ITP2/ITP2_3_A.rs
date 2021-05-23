#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let (mut a, mut b, mut c) = default();
    scanf!("{:i} {:i} {:i}", &mut a, &mut b, &mut c);
    printf!(
        "{:i} {:i}\n",
        Ord::min(Ord::min(a, b), c),
        Ord::max(Ord::max(a, b), c)
    );
}
