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
        let (mut com, mut b, mut e) = default();
        scanf!("{:u} {:usize} {:usize}", &mut com, &mut b, &mut e);
        let slice = &list::slice(a, b..e);
        let it = list::iter(slice);

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
