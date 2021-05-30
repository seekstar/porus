#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let (mut n, mut q) = default();
    scanf!("{:usize} {:u}", &mut n, &mut q);
    let a = &mut VecDeque::new();

    for _ in 0..n {
        let name: String = StringBuffer::with_capacity(11).scan();
        let mut time = default();
        scanf!("{:u}\n", &mut time);
        deque::push_back(a, (name, time));
    }

    let mut sum = default();
    scanf!("{:u}\n", &mut sum);

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
