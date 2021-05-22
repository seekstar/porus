#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<i64> = &mut (0..n).map(|_| read!()).collect();

    let count = list::selection_sort(a, PartialOrd::lt);

    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n{:usize}\n", count);
}
