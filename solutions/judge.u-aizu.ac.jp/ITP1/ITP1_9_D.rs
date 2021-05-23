#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut s = [0u8; 1001];
    scanf!("{:s}", s.as_mut_slice());
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let sc: String = StringBuffer::new(8).scan();
        let command = sc.as_ref();
        let (mut a, mut b) = default();
        scanf!("{:usize} {:usize}", &mut a, &mut b);

        if command == b"replace" {
            let mut p = [0u8; 1001];
            scanf!(" {:s}", p.as_mut_slice());
            s.as_mut_slice()[a..=b].swap_with_slice(p.as_mut_slice()[..=b - a].as_mut());
        } else if command == b"reverse" {
            s.as_mut()[a..=b].reverse();
        } else if command == b"print" {
            printf!("{}\n", s.as_ref()[a..=b].as_ref());
        }
    }
}
