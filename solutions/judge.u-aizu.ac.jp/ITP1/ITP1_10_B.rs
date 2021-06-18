fn main() {
    let (mut a, mut b, mut c) = default();
    scanf!("{:f} {:f} {:f}", &mut a, &mut b, &mut c);

    let d = f64::to_radians(c);
    let h = b * sin(d);
    let w = b * cos(d);

    printf!("{:.5f}\n", a * h / 2.0);
    printf!("{:.5f}\n", (a + b + sqrt(h * h + (a - w) * (a - w))));
    printf!("{:.5f}\n", h);
}
