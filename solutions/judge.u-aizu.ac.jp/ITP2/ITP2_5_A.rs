fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![(0, 0); n];
    for e in List::iter_mut(a) {
        scanf!("{:i} {:i}", &mut e.0, &mut e.1);
    }

    List::shell_sort(
        a,
        PartialOrd::lt,
        &[
            797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1,
        ],
    );

    for (x, y) in List::iter(a) {
        printf!("{:i} {:i}\n", x, y)
    }
}
