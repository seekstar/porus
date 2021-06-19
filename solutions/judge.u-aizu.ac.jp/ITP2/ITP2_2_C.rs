fn main() {
    let (mut n, mut q) = default();
    scanf!("{:usize} {:usize}", &mut n, &mut q);

    let a: &mut Vec<_> = &mut (0..n).map(|_| BinaryHeap::new()).collect();

    for _ in 0..q {
        let (mut op, mut t) = default();
        scanf!("{:u} {:usize}", &mut op, &mut t);
        let h = List::get_mut(a, t).unwrap();
        if op == 0 {
            let mut x = default();
            scanf!("{:i}", &mut x);
            heap::push(h, x);
        } else if op == 1 {
            if let Some(&x) = Heap::peek(h) {
                printf!("{x:i}\n")
            }
        } else if op == 2 {
            Heap::pop(h);
        }
    }
}
