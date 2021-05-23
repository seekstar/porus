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
        let (mut b, mut e, mut t) = default();
        scanf!("{:usize} {:usize} {:usize}", &mut b, &mut e, &mut t);

        for i in b..e {
            list::swap(a, i, t);
            t += 1;
        }
    }

    interleave(list::iter(a), || printf!(" "), |e| printf!("{e:i}"));
    printf!("\n");
}
