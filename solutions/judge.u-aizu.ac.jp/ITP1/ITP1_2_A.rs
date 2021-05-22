#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: i64 = read!();
    let b: i64 = read!();
    printf!(
        "a {} b",
        match Ord::cmp(&a, &b) {
            Less => "<",
            Equal => "==",
            Greater => ">",
        }
    );
}
