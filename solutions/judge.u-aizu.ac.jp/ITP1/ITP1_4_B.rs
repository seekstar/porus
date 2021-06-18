fn main() {
    let mut r = default();
    scanf!("{:f}", &mut r);
    let area = PI * r * r;
    let circumference = PI * 2.0 * r;
    printf!("{area:.6f} {circumference:.6f}\n");
}
