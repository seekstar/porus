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
        let (mut b, mut m, mut e) = default();
        scanf!("{:usize} {:usize} {:usize}", &mut b, &mut m, &mut e);

        List::rotate_left(&mut List::slice_mut(a, b..e), e - m);
    }

    interleave(List::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n");
}
