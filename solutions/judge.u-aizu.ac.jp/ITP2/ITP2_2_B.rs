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
            deque::push_back(List::get_mut(a, t).unwrap(), x);
        } else if op == 1 {
            if !deque::is_empty(List::get(a, t).unwrap()) {
                printf!("{:i}\n", *deque::front(List::get(a, t).unwrap()))
            }
        } else if op == 2 {
            Deque::pop_front(List::get_mut(a, t).unwrap());
        }
    }
}
