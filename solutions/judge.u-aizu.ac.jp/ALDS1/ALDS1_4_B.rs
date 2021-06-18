fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let s = &mut vec![default(); n];
    for e in list::iter_mut(s) {
        scanf!("{:u}", e);
    }
    let mut q = default();
    scanf!("{:u}", &mut q);

    let mut count: usize = 0;

    for _ in 0..q {
        let mut t = default();
        scanf!("{:u}", &mut t);
        let r = list::bsearch(s, &t);
        if r.1 > r.0 {
            count += 1;
        }
    }

    printf!("{:usize}\n", count);
}
