#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: u64 = read!();

    'outer: for i in 1..=n {
        if (i % 3) != 0 {
            let mut x = i;
            while (x % 10) != 3 {
                if x == 0 {
                    continue 'outer;
                }
                x /= 10;
            }
        }

        printf!(" {i:u}");
    }
    printf!("\n");
}
