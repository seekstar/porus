#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let _n: usize = read!();
    let k: usize = read!();
    let b: Vec<u64> = (0..k).map(|_| read!()).collect();

    for i in 0..1 << k {
        printf!(
            "{:u}:",
            (0..k).fold(0, |sum, d| if ((1 << d) & i) > 0 {
                sum | (1u64 << b[d])
            } else {
                sum
            })
        );
        for j in 0..k {
            if ((1 << j) & i) > 0 {
                printf!(" {:u}", b[j])
            }
        }
        printf!("\n");
    }
}
