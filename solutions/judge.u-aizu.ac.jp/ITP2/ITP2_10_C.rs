#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut state: u64 = 0;
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let mut query = default();
        scanf!("{:u}", &mut query);

        match query {
            0 => {
                let mut i = default();
                scanf!("{:usize}", &mut i);
                printf!("{:u}\n", (state & (1 << i)) > 0)
            }
            1 => {
                let mut i = default();
                scanf!("{:usize}", &mut i);
                state |= 1 << i;
            }
            2 => {
                let mut i = default();
                scanf!("{:usize}", &mut i);
                state &= !(1 << i);
            }
            3 => {
                let mut i = default();
                scanf!("{:usize}", &mut i);
                state ^= 1 << i;
            }
            4 => {
                printf!("{:u}\n", state == u64::max_value());
            }
            5 => {
                printf!("{:u}\n", state > 0);
            }
            6 => {
                printf!("{:u}\n", state == 0);
            }
            7 => {
                printf!("{:u}\n", state.count_ones());
            }
            8 => {
                printf!("{:u}\n", state);
            }
            _ => panic!("invalid query"),
        }
    }
}
