#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![(0, 0); n];
    for e in a.iter_mut() {
        scanf!("{:i} {:i}", &mut e.0, &mut e.1);
    }

    list::shell_sort(
        a,
        PartialOrd::lt,
        &[
            797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1,
        ],
    );

    for (x, y) in list::iter(a) {
        printf!("{:i} {:i}\n", x, y)
    }
}
