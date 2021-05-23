#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in a.iter_mut() {
        scanf!("{:i}", e);
    }

    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n");

    for i in 2..n + 1 {
        sorting::bubble_sorted(&mut list::slice_mut(a, ..i), PartialOrd::lt);
        interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
        printf!("\n");
    }
}
