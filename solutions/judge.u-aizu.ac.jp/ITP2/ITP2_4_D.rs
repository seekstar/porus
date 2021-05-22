#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &Vec<i64> = &(0..n).map(|_| read!()).collect();
    let mut old = None;

    interleave(
        list::iter(a).filter_map(|x| {
            if old == Some(x) {
                None
            } else {
                old = Some(x);
                Some(x)
            }
        }),
        || printf!(" "),
        |e| printf!("{e:i}"),
    );
    printf!("\n");
}
