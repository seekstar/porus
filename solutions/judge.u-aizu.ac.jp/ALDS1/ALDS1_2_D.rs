fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:u}", e);
    }

    let gaps = &[
        797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1,
    ];

    let mut skip = 0;
    while (List::get(gaps, skip).unwrap() > &n) && (List::get(gaps, skip).unwrap() != &1) {
        skip += 1;
    }
    let g = &List::slice(gaps, skip..);

    let count = List::shell_sort(a, PartialOrd::lt, g);

    printf!("{:usize}\n", Collection::size(g));
    interleave(List::iter(g), || printf!(" "), |e| printf!("{e:usize}"));

    printf!("\n{:usize}\n", count);
    for e in List::iter(a) {
        printf!("{e:u}\n");
    }
}
