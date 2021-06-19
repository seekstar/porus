fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:i}", e);
    }

    let count = List::selection_sort(a, PartialOrd::lt);
    interleave(List::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n{:usize}\n", count);
}
