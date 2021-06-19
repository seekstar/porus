fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a: &mut Vec<(u64, u64, u8, u64, String)> = &mut (0..n)
        .map(|_| {
            let (mut v, mut w, mut t, mut d) = default();
            scanf!("{:u} {:u} {:c} {:u}", &mut v, &mut w, &mut t, &mut d);
            let s: String = StringBuffer::with_capacity(21).scan();
            (v, w, t, d, s)
        })
        .collect();

    List::shell_sort(
        a,
        PartialOrd::lt,
        &[
            797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1,
        ],
    );

    for (v, w, t, d, s) in List::iter(a) {
        printf!("{:u} {:u} {:c} {:u} {:s}\n", v, w, t, d, &s);
    }
}
