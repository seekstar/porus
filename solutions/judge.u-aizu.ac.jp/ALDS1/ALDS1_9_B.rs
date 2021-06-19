fn main() {
    let mut h = default();
    scanf!("{:usize}", &mut h);
    let v = &mut vec![default(); h];
    for e in List::iter_mut(v) {
        scanf!("{:i}", e);
    }
    dheap::heapify(2, v, PartialOrd::gt);

    for e in List::iter(v) {
        printf!(" {:i}", e);
    }
    printf!("\n");
}
