fn main() {
    let mut h = default();
    scanf!("{:usize}", &mut h);
    let v = &mut vec![default(); h];
    for e in list::iter_mut(v) {
        scanf!("{:i}", e);
    }
    dheap::heapify(2, v, PartialOrd::gt);

    list::iter(v).for_each(|e| printf!(" {:i}", e));
    printf!("\n");
}
