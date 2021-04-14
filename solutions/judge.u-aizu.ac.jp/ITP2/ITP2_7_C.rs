#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let q: usize = read!();
    let s = &mut BTreeSet::new();

    for _ in 0..q {
        let query: usize = read!();

        match query {
            0 => {
                let x: usize = read!();
                set::insert(s, x);
                writelnf!("{:d}", collection::size(s));
            }
            1 => {
                let x: usize = read!();
                writelnf!("{:d}", if set::contains(s, &x) { 1 } else { 0 });
            }
            2 => {
                let x: usize = read!();
                set::remove(s, &x);
            }
            3 => {
                let l: usize = read!();
                let r: usize = read!();
                for c in set::range(s, l..=r) {
                    writelnf!("{:d}", c);
                }
            }
            _ => panic!("invalid query"),
        }
    }
}
