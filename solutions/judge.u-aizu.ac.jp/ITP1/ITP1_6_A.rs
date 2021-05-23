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
    interleave(list::iter(a).rev(), || printf!(" "), |e| printf!("{e:u}"));
    printf!("\n");
}
