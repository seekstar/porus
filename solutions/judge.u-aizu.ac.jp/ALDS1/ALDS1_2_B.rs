fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in list::iter_mut(a) {
        scanf!("{:i}", e);
    }

    let count = list::selection_sort(a, PartialOrd::lt);
    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n{:usize}\n", count);
}
