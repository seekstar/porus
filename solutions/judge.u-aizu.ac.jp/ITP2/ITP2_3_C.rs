fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:i}", e);
    }
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let (mut b, mut e, mut k) = default();
        scanf!("{:usize} {:usize} {:i}", &mut b, &mut e, &mut k);
        let slice = &List::slice(a, b..e);
        let it = List::iter(slice);

        printf!("{:usize}\n", it.filter(|&x| x == k).count())
    }
}
