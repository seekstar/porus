#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let k: usize = read!();
    let b: u64 = (0..k).map(|_| read!()).fold(0, |b, i: u64| b | (1 << i));

    for i in 0..1 << n {
        if (i & b) != b {
            continue;
        }
        printf!("{:u}:", i);
        for j in 0..n {
            if ((1 << j) & i) > 0 {
                printf!(" {:usize}", j)
            }
        }
        printf!("\n");
    }
}
