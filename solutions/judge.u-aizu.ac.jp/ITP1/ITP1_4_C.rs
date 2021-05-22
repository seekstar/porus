#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let (mut a, mut op, mut b): (i64, u8, i64) = default();
        read!(&mut a, Char(&mut op), &mut b);

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
