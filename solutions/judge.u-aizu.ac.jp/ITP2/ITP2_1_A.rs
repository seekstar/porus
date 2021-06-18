fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let vec = &mut vec![0; 0];

    for _ in 0..n {
        let mut op = default();
        scanf!("{:u}", &mut op);
        if op == 0 {
            let mut x = default();
            scanf!("{:i}", &mut x);
            stack::push(vec, x);
        } else if op == 1 {
            let mut p = default();
            scanf!("{:usize}", &mut p);
            printf!("{:i}\n", *list::get(vec, p));
        } else if op == 2 {
            stack::pop(vec);
        }
    }
}
