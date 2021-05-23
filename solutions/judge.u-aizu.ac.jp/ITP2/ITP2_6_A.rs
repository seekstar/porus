#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in a.iter_mut() {
        scanf!("{:u}", e);
    }
    let mut q = default();
    scanf!("{:usize}", &mut q);

    for _ in 0..q {
        let mut k = default();
        scanf!("{:u}", &mut k);
        let r = list::bsearch(a, &k);
        printf!("{:u}\n", r.1 > r.0);
    }
}
