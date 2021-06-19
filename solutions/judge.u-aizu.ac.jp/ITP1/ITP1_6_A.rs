fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:u}", e);
    }
    interleave(
        Iterator::rev(List::iter(a)),
        || printf!(" "),
        |e| printf!("{e:u}"),
    );
    printf!("\n");
}
