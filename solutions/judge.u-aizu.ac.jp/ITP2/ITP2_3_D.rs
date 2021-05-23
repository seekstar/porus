#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in a.iter_mut() {
        scanf!("{:u}", e);
    }
    let mut m = default();
    scanf!("{:usize}", &mut m);
    let b = &mut vec![default(); m];
    for e in b.iter_mut() {
        scanf!("{:u}", e);
    }

    printf!("{:u}\n", b.iter().cmp(a.iter()) == Greater);
}
