#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut state: u64 = 0;
    let n: usize = read!();

    let mask: Vec<u64> = (0..n)
        .map(|_| {
            let k: usize = read!();
            (0..k)
                .map(|_| read!())
                .fold(0, |sum, b: u64| sum | (1 << b))
        })
        .collect();

    let q: usize = read!();
    for _ in 0..q {
        let query: usize = read!();
        let m: usize = read!();

        match query {
            0 => {
                printf!("{:u}\n", (state & (1 << m)) > 0);
            }
            1 => {
                state |= mask[m];
            }
            2 => {
                state &= !mask[m];
            }
            3 => {
                state ^= mask[m];
            }
            4 => {
                printf!("{:u}\n", (state & mask[m]) == mask[m]);
            }
            5 => {
                printf!("{:u}\n", (state & mask[m]) > 0);
            }
            6 => {
                printf!("{:u}\n", (state & mask[m]) == 0);
            }
            7 => {
                printf!("{:u}\n", (state & mask[m]).count_ones());
            }
            8 => {
                printf!("{:u}\n", state & mask[m]);
            }
            _ => panic!("invalid query"),
        }
    }
}
