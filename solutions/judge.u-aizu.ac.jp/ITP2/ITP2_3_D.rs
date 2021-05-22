#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &Vec<u64> = &(0..n).map(|_| read!()).collect();
    let m: usize = read!();
    let b: &Vec<u64> = &(0..m).map(|_| read!()).collect();

    printf!("{:u}\n", b.iter().cmp(a.iter()) == Greater);
}
