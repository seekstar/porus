#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<u64> = &mut (0..n).map(|_| read!()).collect();
    let q: usize = read!();

    for _ in 0..q {
        let k: u64 = read!();
        let r = list::bsearch(a, &k);
        printf!("{:usize}\n", r.0);
    }
}
