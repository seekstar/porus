#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let (mut a, mut b, mut c) = default();
    scanf!("{:u} {:u} {:u}", &mut a, &mut b, &mut c);
    printf!("{:usize}\n", (a..=b).filter(|x| (&c) % x == 0).count());
}
