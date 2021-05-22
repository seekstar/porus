#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let h: usize = read!();
    let v: &mut Vec<i64> = &mut (0..h).map(|_| read!()).collect();
    dheap::heapify(2, v, PartialOrd::gt);

    list::iter(v).for_each(|e| printf!(" {:i}", e));
    printf!("\n");
}
