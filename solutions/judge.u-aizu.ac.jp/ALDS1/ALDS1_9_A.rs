#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut h = default();
    scanf!("{:usize}", &mut h);
    let v = &mut vec![default(); h];
    for e in v.iter_mut() {
        scanf!("{:i}", e);
    }

    let write_key = |name: &str, i: usize| {
        printf!("{} key = {:i}, ", name, *list::get(v, i));
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
