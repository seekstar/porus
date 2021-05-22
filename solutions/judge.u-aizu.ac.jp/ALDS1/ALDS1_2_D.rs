#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<u64> = &mut (0..n).map(|_| read!()).collect();

    let gaps = &[
        797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1,
    ];

    let mut skip = 0;
    while (list::get(gaps, skip) > &n) && (list::get(gaps, skip) != &1) {
        skip += 1;
    }
    let g = &list::slice(gaps, skip..);

    let count = list::shell_sort(a, PartialOrd::lt, g);

    printf!("{:usize}\n", collection::size(g));
    interleave(list::iter(g), || printf!(" "), |e| printf!("{e:usize}"));

    printf!("\n{:usize}\n", count);
    list::iter(a).for_each(|e| printf!("{e:u}\n"));
}
