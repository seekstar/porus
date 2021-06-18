fn main() {
    let (mut n, mut q) = default();
    scanf!("{:usize} {:usize}", &mut n, &mut q);

    let a: &mut Vec<_> = &mut (0..n).map(|_| VecDeque::<_>::new()).collect();

    for _ in 0..q {
        let (mut op, mut t) = default();
        scanf!("{:u} {:usize}", &mut op, &mut t);
        if op == 0 {
            let mut x = default();
            scanf!("{:i}", &mut x);
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
