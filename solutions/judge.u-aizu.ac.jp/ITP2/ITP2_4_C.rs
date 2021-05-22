#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<i64> = &mut (0..n).map(|_| read!()).collect();
    let q: usize = read!();

    for _ in 0..q {
        let b: usize = read!();
        let e: usize = read!();
        let mut t: usize = read!();

        for i in b..e {
            list::swap(a, i, t);
            t += 1;
        }
    }

    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n");
}
