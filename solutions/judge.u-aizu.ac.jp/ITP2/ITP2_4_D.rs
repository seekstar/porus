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
