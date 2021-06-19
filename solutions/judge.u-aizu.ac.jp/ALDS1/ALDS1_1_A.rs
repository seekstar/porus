fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:i}", e);
    }

    interleave(List::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n");

    for i in 2..n + 1 {
        List::bubble_sorted(&mut List::slice_mut(a, ..i), PartialOrd::lt);
        interleave(List::iter(a), || printf!(" "), |e| printf!("{e:i}"));
        printf!("\n");
    }
}
