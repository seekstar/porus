fn main() {
    let (mut w, mut h, mut x, mut y, mut r) = default();
    scanf!(
        "{:i} {:i} {:i} {:i} {:i}",
        &mut w,
        &mut h,
        &mut x,
        &mut y,
        &mut r
    );
    printf!(
        "{}",
        if (r <= x) && (x <= (w - r)) && (r <= y) && (y <= (h - r)) {
            "Yes"
        } else {
            "No"
        }
    );
}
