#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();

    let buf = &mut VecDeque::new();

    for _ in 0..n {
        let op: u64 = read!();
        if op == 0 {
            let d: usize = read!();
            let x: i64 = read!();
            if d == 0 {
                deque::push_front(buf, x);
            } else if d == 1 {
                deque::push_back(buf, x);
            }
        } else if op == 1 {
            let p: usize = read!();
            printf!("{:i}\n", *list::get(buf, p));
        } else if op == 2 {
            let d: u64 = read!();
            if d == 0 {
                deque::pop_front(buf);
            } else if d == 1 {
                deque::pop_back(buf);
            }
        }
    }
}
