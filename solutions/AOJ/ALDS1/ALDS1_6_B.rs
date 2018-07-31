#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n : isize = read!();
    let a = &mut Array::<isize>::new_from_iter((0..n).map(|_| read!()));

    let pivot = list::sort::partition(a, &PartialOrd::le);

    writelnf!(
        "{}[{:d}]{}",
        join(f!(""), list::iter(slice!(a, [, pivot])).map(|e| f!("{e:d} "))),
        a[pivot],
        join(f!(""), list::iter(slice!(a, [pivot+1, ])).map(|e| f!(" {e:d}"))));
}
