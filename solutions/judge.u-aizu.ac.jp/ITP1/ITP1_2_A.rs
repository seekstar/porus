#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let (mut a, mut b) = default();
    scanf!("{:i} {:i}", &mut a, &mut b);
    printf!(
        "a {} b",
        match Ord::cmp(&a, &b) {
            Less => "<",
            Equal => "==",
            Greater => ">",
        }
    );
}
