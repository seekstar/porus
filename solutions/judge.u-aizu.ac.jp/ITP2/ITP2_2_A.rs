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
            stack::push(List::get_mut(a, t).unwrap(), x);
        } else if op == 1 {
            if !stack::is_empty(List::get(a, t).unwrap()) {
                printf!("{:i}\n", *stack::top(List::get(a, t).unwrap()))
            }
        } else if op == 2 {
            Stack::pop(List::get_mut(a, t).unwrap());
        }
    }
}
