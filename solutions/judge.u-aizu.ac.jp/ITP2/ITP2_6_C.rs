fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in list::iter_mut(a) {
        scanf!("{:u}", e);
    }
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let mut k = default();
        scanf!("{:u}", &mut k);
        let r = list::bsearch(a, &k);
        printf!("{:usize}\n", r.0);
    }
}
