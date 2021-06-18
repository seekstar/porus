fn main() {
    let (mut n, mut k) = default();
    scanf!("{:usize} {:usize}", &mut n, &mut k);

    let b: u64 = (0..k)
        .map(|_| {
            let mut e = default();
            scanf!("{:usize}", &mut e);
            e
        })
        .fold(0, |b, i| b | (1 << i));

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
