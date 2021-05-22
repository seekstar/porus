#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<u64> = &mut (0..n).map(|_| read!()).collect();

    let pivot = sorting::partition(a, PartialOrd::le);

    let l = &list::slice(a, ..pivot);
    let r = &list::slice(a, (pivot + 1)..);

    list::iter(l).for_each(|e| printf!("{e:u} "));
    printf!("[{:u}]", *list::get(a, pivot));
    list::iter(r).for_each(|e| printf!(" {e:u}"));
}
