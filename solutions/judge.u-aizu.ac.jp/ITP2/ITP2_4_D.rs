fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:i}", e);
    }
    let mut old = None;

    interleave(
        List::iter(a).filter_map(|x| {
            if old == Some(x) {
                None
            } else {
                old = Some(x);
                Some(x)
            }
        }),
        || printf!(" "),
        |e| printf!("{e:i}"),
    );
    printf!("\n");
}
