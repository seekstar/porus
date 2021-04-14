#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let q: usize = read!();
    let s = &mut BTreeSet::new();

    for _ in 0..q {
        let query: usize = read!();
        let x: usize = read!();

        match query {
            0 => {
                set::insert(s, x);
                writelnf!("{:d}", collection::size(s));
            }
            1 => {
                writelnf!("{:d}", if set::contains(s, &x) { 1 } else { 0 });
            }
            2 => {
                set::remove(s, &x);
            }
            _ => panic!("invalid query"),
        }
    }
}
