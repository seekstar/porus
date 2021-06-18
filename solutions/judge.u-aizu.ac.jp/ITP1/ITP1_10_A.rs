fn main() {
    let (mut x1, mut y1, mut x2, mut y2) = default();
    scanf!("{:f} {:f} {:f} {:f}", &mut x1, &mut y1, &mut x2, &mut y2);
    printf!(
        "{:.5f}\n",
        sqrt((y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1))
    );
}
