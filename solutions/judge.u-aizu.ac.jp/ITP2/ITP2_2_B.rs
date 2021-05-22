#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a: &mut Vec<_> = &mut (0..n).map(|_| VecDeque::<_>::new()).collect();

    for _ in 0..q {
        let op: u64 = read!();
        let t: usize = read!();
        if op == 0 {
            let x: i64 = read!();
            deque::push_back(list::get_mut(a, t), x);
        } else if op == 1 {
            if !deque::is_empty(list::get(a, t)) {
                printf!("{:i}\n", *deque::front(list::get(a, t)))
            }
        } else if op == 2 {
            Deque::pop_front(list::get_mut(a, t));
        }
    }
}
