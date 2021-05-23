#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let (mut x, mut y) = default();
        scanf!("{:u} {:u}", &mut x, &mut y);
        if (x == 0) && (y == 0) {
            break;
        }
        printf!("{:u} {:u}\n", Ord::min(x, y), Ord::max(x, y));
    }
}
