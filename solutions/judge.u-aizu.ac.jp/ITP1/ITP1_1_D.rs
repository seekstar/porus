#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut t = default();
    scanf!("{:u}", &mut t);
    let s = t % 60;
    let mut m = t / 60;
    let h = m / 60;
    m = m % 60;
    printf!("{h:u}:{m:u}:{s:u}\n");
}
