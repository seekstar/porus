fn main() {
    let h = &mut BinaryHeap::new();

    loop {
        let s: String = StringBuffer::with_capacity(8).scan();
        let command = s.as_ref();
        if command == b"end" {
            break;
        } else if command == b"insert" {
            let mut k = default();
            scanf!("{:u}", &mut k);
            heap::push(h, k);
        } else if command == b"extract" {
            printf!("{:u}\n", heap::pop(h));
        }
    }
}
