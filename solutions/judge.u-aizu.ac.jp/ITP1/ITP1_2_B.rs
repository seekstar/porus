fn main() {
    let (mut a, mut b, mut c) = default();
    scanf!("{:u} {:u} {:u}", &mut a, &mut b, &mut c);
    printf!("{}", if (a < b) && (b < c) { "Yes" } else { "No" });
}
