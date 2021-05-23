#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);

    for i in 0u64..1 << n {
        printf!("{:u}:", i);
        for j in 0..n {
            if ((1 << j) & i) > 0 {
                printf!(" {:usize}", j)
            }
        }
        printf!("\n");
    }
}
