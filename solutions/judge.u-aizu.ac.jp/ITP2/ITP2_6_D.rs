fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:u}", e);
    }
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let mut k = default();
        scanf!("{:u}", &mut k);
        let r = List::bsearch(a, &k);
        printf!("{:usize} {:usize}\n", r.start, r.end);
    }
}
