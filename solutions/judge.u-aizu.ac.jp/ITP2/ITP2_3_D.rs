#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &Vec<usize> = &(0..n).map(|_| read!()).collect();
    let m: usize = read!();
    let b: &Vec<usize> = &(0..m).map(|_| read!()).collect();

    writelnf!(
        "{:d}",
        if b.iter().cmp(a.iter()) == Greater {
            1
        } else {
            0
        }
    );
}
