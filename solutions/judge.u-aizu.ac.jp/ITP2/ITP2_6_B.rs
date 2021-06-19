fn includes(mut a: impl Iterator<Item = i64>, b: impl Iterator<Item = i64>) -> bool {
    let mut x = 0;
    for y in b {
        loop {
            if let Some(z) = a.next() {
                x = z;
            } else {
                return false;
            }

            if y < x {
                return false;
            }

            if y == x {
                break;
            }
        }
    }

    return true;
}

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:i}", e);
    }

    let mut m = default();
    scanf!("{:usize}", &mut m);
    let b = &mut vec![default(); m];
    for e in List::iter_mut(b) {
        scanf!("{:i}", e);
    }

    printf!("{:u}\n", includes(List::iter(a), List::iter(b)));
}
