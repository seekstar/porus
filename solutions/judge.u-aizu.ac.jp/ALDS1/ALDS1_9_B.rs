#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut h = default();
    scanf!("{:usize}", &mut h);
    let v = &mut vec![default(); h];
    for e in v.iter_mut() {
        scanf!("{:i}", e);
    }
    dheap::heapify(2, v, PartialOrd::gt);

    list::iter(v).for_each(|e| printf!(" {:i}", e));
    printf!("\n");
}
