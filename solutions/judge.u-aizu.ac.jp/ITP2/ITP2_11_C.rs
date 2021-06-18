fn main() {
    let (mut n, mut k) = default();
    scanf!("{:usize} {:usize}", &mut n, &mut k);
    let b = &mut vec![default(); k];
    for e in list::iter_mut(b) {
        scanf!("{:u}", e);
    }

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
