#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let x: u64 = read!();
        let y: u64 = read!();
        if (x == 0) && (y == 0) {
            break;
        }
        printf!("{:u} {:u}\n", Ord::min(x, y), Ord::max(x, y));
    }
}
