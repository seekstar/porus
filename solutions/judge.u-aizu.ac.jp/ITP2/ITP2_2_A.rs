fn main() {
    let (mut n, mut q) = default();
    scanf!("{:usize} {:usize}", &mut n, &mut q);

    let a: &mut Vec<_> = &mut (0..n).map(|_| Vec::new()).collect();

    for _ in 0..q {
        let (mut op, mut t) = default();
        scanf!("{:u} {:usize}", &mut op, &mut t);
        if op == 0 {
            let mut x = default();
            scanf!("{:i}", &mut x);
            stack::push(list::get_mut(a, t), x);
        } else if op == 1 {
            if !stack::is_empty(list::get(a, t)) {
                printf!("{:i}\n", *stack::top(list::get(a, t)))
            }
        } else if op == 2 {
            Stack::pop(list::get_mut(a, t));
        }
    }
}
