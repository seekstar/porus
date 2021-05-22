#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: u64 = read!();

    let a = &mut VecDeque::new();

    for _ in 0..n {
        let b: StringBuffer = read!();
        let name: String = From::from(b);
        let time: u64 = read!();
        deque::push_back(a, (name, time));
    }

    let mut sum: u64 = 0;

    while !deque::is_empty(a) {
        let (name, time) = deque::pop_front(a);
        if time <= q {
            sum += time;
            printf!("{} {:u}\n", &name, sum);
        } else {
            sum += q;
            deque::push_back(a, (name, time - q));
        }
    }
}
