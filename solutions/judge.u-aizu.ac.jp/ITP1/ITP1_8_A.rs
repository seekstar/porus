fn main() {
    let mut c = default();
    while scanf!("{:c}", &mut c) {
        printf!(
            "{:c}",
            match c {
                b'a'..=b'z' => c - b'a' + b'A',
                b'A'..=b'Z' => c - b'A' + b'a',
                _ => c,
            }
        );
    }
}
