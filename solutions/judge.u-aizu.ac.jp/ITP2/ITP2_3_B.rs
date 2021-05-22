#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &Vec<i64> = &(0..n).map(|_| read!()).collect();
    let q: usize = read!();

    for _ in 0..q {
        let com: u64 = read!();
        let b: usize = read!();
        let e: usize = read!();
        let slice = &list::slice(a, b..e);
        let it = list::iter(slice);

        printf!(
            "{:i}\n",
            if com == 0 {
                it.min().unwrap()
            } else if com == 1 {
                it.max().unwrap()
            } else {
                panic!();
            }
        )
    }
}
