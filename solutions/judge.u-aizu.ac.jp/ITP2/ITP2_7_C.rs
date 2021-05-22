#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let q: usize = read!();
    let s = &mut BTreeSet::new();

    for _ in 0..q {
        let query: u64 = read!();

        match query {
            0 => {
                let x: u64 = read!();
                set::insert(s, x);
                printf!("{:usize}\n", collection::size(s));
            }
            1 => {
                let x: u64 = read!();
                printf!("{:u}\n", set::contains(s, &x));
            }
            2 => {
                let x: u64 = read!();
                set::remove(s, &x);
            }
            3 => {
                let l: u64 = read!();
                let r: u64 = read!();
                for c in set::range(s, l..=r) {
                    printf!("{:u}\n", *c);
                }
            }
            _ => panic!("invalid query"),
        }
    }
}
