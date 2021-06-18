fn main() {
    let (mut a, mut b) = default();
    scanf!("{:u} {:u}", &mut a, &mut b);
    let d = a / b;
    let r = a % b;
    let f = (a as f64) / (b as f64);
    printf!("{d:u} {r:u} {f:.6f}\n");
}
