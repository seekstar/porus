fn main() {
    loop {
        let (mut a, mut op, mut b) = default();
        scanf!("{:i} {:c} {:i}", &mut a, &mut op, &mut b);

        if op == b'?' {
            break;
        }

        printf!(
            "{:i}\n",
            match op {
                b'+' => a + b,
                b'-' => a - b,
                b'*' => a * b,
                b'/' => a / b,
                _ => panic!(),
            }
        );
    }
}
