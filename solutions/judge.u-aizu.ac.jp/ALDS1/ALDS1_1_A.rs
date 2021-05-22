#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<i64> = &mut (0..n).map(|_| read!()).collect();

    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n");

    for i in 2..n + 1 {
        sorting::bubble_sorted(&mut list::slice_mut(a, ..i), PartialOrd::lt);
        interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
        printf!("\n");
    }
}
