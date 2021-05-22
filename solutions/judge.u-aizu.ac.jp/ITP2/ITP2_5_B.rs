#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<(u64, u64, u8, u64, String)> = &mut (0..n)
        .map(|_| {
            let v = read!();
            let w = read!();
            let mut t: u8 = 0;
            read!(Char(&mut t));
            let d = read!();
            let b: StringBuffer = read!();
            let s: String = From::from(b);
            (v, w, t, d, s)
        })
        .collect();

    list::shell_sort(
        a,
        PartialOrd::lt,
        &[
            797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1,
        ],
    );

    for (v, w, t, d, s) in list::iter(a) {
        printf!("{:u} {:u} {:c} {:u} {:s}\n", v, w, t, d, &s);
    }
}
