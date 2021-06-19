fn main() {
    let mut h = default();
    scanf!("{:usize}", &mut h);
    let v = &mut vec![default(); h];
    for e in List::iter_mut(v) {
        scanf!("{:i}", e);
    }

    let write_key = |name: &str, i: usize| {
        printf!("{} key = {:i}, ", name, *List::get(v, i).unwrap());
    };

    for i in 0..h {
        printf!("node {:usize}:", i + 1);
        write_key("", i);
        if let Some(parent) = dheap::parent_index(2, i) {
            write_key("parent", parent);
        }
        let left = dheap::child_index(2, i, 0);
        let right = dheap::child_index(2, i, 1);
        if left < h {
            write_key("left", left);
        }
        if right < h {
            write_key("right", right);
        }
        printf!("\n");
    }
}
