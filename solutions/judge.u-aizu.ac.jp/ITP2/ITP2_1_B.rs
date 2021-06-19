fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);

    let buf = &mut VecDeque::new();

    for _ in 0..n {
        let mut op = default();
        scanf!("{:u}", &mut op);
        if op == 0 {
            let (mut d, mut x) = default();
            scanf!("{:u} {:i}", &mut d, &mut x);
            if d == 0 {
                deque::push_front(buf, x);
            } else if d == 1 {
                deque::push_back(buf, x);
            }
        } else if op == 1 {
            let mut p = default();
            scanf!("{:usize}", &mut p);
            printf!("{:i}\n", *List::get(buf, p).unwrap());
        } else if op == 2 {
            let mut d = default();
            scanf!("{:u}", &mut d);
            if d == 0 {
                deque::pop_front(buf);
            } else if d == 1 {
                deque::pop_back(buf);
            }
        }
    }
}
