fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in list::iter_mut(a) {
        scanf!("{:u}", e);
    }
    let mut m = default();
    scanf!("{:usize}", &mut m);
    let b = &mut vec![default(); m];
    for e in b.iter_mut() {
        scanf!("{:u}", e);
    }

    printf!("{:u}\n", b.iter().cmp(a.iter()) == Greater);
}
