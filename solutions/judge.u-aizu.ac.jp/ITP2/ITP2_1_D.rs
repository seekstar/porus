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
            interleave(
                list::iter(list::get(a, t)),
                || printf!(" "),
                |e| printf!("{e:i}"),
            );
            printf!("\n");
        } else if op == 2 {
            list::set(a, t, Vec::new());
        }
    }
}
