fn main() {
    let mut state: u64 = 0;
    let mut n = default();
    scanf!("{:usize}", &mut n);

    let mask: Vec<u64> = (0..n)
        .map(|_| {
            let mut k = default();
            scanf!("{:usize}", &mut k);
            (0..k)
                .map(|_| {
                    let mut e = default();
                    scanf!("{:u}", &mut e);
                    e
                })
                .fold(0, |sum, b| sum | (1 << b))
        })
        .collect();

    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let (mut query, mut m) = default();
        scanf!("{:u} {:usize}", &mut query, &mut m);

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
