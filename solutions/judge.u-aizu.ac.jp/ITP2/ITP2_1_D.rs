#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a: &mut Vec<_> = &mut (0..n).map(|_| Vec::new()).collect();

    for _ in 0..q {
        let op: u64 = read!();
        let t: usize = read!();
        if op == 0 {
            let x: i64 = read!();
            stack::push(list::get_mut(a, t), x);
        } else if op == 1 {
            interleave(
                list::iter(list::get(a, t)),
                || printf!(" "),
                |e| printf!("{e:i}"),
            );
            printf!("\n");
        } else if op == 2 {
            list::set(a, t, Vec::new());
        }
    }
}
