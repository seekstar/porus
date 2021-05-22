#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let q: usize = read!();
    let s = &mut BTreeSet::new();

    for _ in 0..q {
        let query: u64 = read!();
        let x: u64 = read!();

        match query {
            0 => {
                set::insert(s, x);
                printf!("{:usize}\n", collection::size(s));
            }
            1 => {
                printf!("{:u}\n", set::contains(s, &x));
            }
            _ => panic!("invalid query"),
        }
    }
}
