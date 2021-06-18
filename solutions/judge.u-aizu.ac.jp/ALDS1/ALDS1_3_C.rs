fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);

    let a = &mut DoublyLinkedList::new_in(Chunk::<_>::with_capacity(1000000));

    for _ in 0..n {
        let s: String = StringBuffer::with_capacity(12).scan();
        let command = s.as_ref();
        if command == b"insert" {
            let mut x = default();
            scanf!("{:u}", &mut x);
            a.insert_after(x, None);
        } else if command == b"delete" {
            let mut x = default();
            scanf!("{:u}", &mut x);
            let mut h = a.front();
            while !h.is_none() {
                if a.get(h.unwrap()) == &x {
                    a.remove(h.unwrap());
                    break;
                }
                h = a.next(h);
            }
        } else if command == b"deleteFirst" {
            let h = a.front().unwrap();
            a.remove(h);
        } else if command == b"deleteLast" {
            let h = a.back().unwrap();
            a.remove(h);
        }
    }

    interleave(deque::drain(a), || printf!(" "), |e| printf!("{e:u}"));
    printf!("\n");
}
