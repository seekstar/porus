#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);

    let mut min = i64::max_value();
    let mut max = i64::min_value();
    let mut sum: i64 = 0;

    for _ in 0..n {
        let mut a = default();
        scanf!("{:i}", &mut a);

        min = Ord::min(a, min);
        max = Ord::max(a, max);
        sum = sum + a;
    }

    printf!("{min:i} {max:i} {sum:i}\n");
}
