#[macro_use]
extern crate porus;
prelude!();

fn main() {
    for _ in 0..1000 {
        printf!("Hello World\n");
    }
}
