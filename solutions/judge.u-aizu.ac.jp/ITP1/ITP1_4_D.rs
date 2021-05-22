#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: u64 = read!();

    let mut min = i64::max_value();
    let mut max = i64::min_value();
    let mut sum: i64 = 0;

    for _ in 0..n {
        let a: i64 = read!();

        min = Ord::min(a, min);
        max = Ord::max(a, max);
        sum = sum + a;
    }

    printf!("{min:i} {max:i} {sum:i}\n");
}
