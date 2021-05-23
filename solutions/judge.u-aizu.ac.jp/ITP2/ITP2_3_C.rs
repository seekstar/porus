#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in a.iter_mut() {
        scanf!("{:i}", e);
    }
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let (mut b, mut e, mut k) = default();
        scanf!("{:usize} {:usize} {:i}", &mut b, &mut e, &mut k);
        let slice = &list::slice(a, b..e);
        let it = list::iter(slice);

        printf!("{:usize}\n", it.filter(|&x| x == k).count())
    }
}
