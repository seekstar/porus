#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let vec = &mut vec![0; 0];

    for _ in 0..n {
        let op: u64 = read!();
        if op == 0 {
            let x: i64 = read!();
            stack::push(vec, x);
        } else if op == 1 {
            let p: usize = read!();
            printf!("{:i}\n", *list::get(vec, p));
        } else if op == 2 {
            stack::pop(vec);
        }
    }
}
