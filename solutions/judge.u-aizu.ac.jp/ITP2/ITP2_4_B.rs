fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in list::iter_mut(a) {
        scanf!("{:i}", e);
    }
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let (mut b, mut m, mut e) = default();
        scanf!("{:usize} {:usize} {:usize}", &mut b, &mut m, &mut e);

        list::rotate_left(&mut list::slice_mut(a, b..e), e - m);
    }

    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n");
}
