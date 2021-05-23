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

    let count = list::selection_sort(a, PartialOrd::lt);
    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n{:usize}\n", count);
}
