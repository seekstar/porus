#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut i: usize = 1;
    loop {
        let mut x = default();
        scanf!("{:u}", &mut x);
        if x == 0 {
            break;
        }
        printf!("Case {:usize}: {:u}\n", i, x);
        i += 1;
    }
}
