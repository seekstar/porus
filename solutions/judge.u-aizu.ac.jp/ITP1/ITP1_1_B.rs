#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut x = default();
    scanf!("{:u}", &mut x);
    let cube = x * x * x;
    printf!("{cube:u}\n");
}
