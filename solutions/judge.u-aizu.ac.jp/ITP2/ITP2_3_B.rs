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
        let (mut com, mut b, mut e) = default();
        scanf!("{:u} {:usize} {:usize}", &mut com, &mut b, &mut e);
        let slice = &List::slice(a, b..e);
        let it = List::iter(slice);

        printf!(
            "{:i}\n",
            if com == 0 {
                it.min().unwrap()
            } else if com == 1 {
                it.max().unwrap()
            } else {
                panic!();
            }
        )
    }
}
