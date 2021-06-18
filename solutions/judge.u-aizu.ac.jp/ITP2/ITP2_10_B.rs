fn print(x: u32) {
    for i in 0..32usize {
        printf!("{:u}", ((x << i) >> 31) & 1);
    }
    printf!("\n");
}

fn main() {
    let (mut a, mut b) = default();
    scanf!("{:u32} {:u32}", &mut a, &mut b);
    print(a & b);
    print(a | b);
    print(a ^ b);
}
