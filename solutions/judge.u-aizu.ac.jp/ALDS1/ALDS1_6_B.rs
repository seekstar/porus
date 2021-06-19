fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in List::iter_mut(a) {
        scanf!("{:u}", e);
    }

    let pivot = List::partition(a, PartialOrd::le);

    let l = &List::slice(a, ..pivot);
    let r = &List::slice(a, (pivot + 1)..);

    for e in List::iter(l) {
        printf!("{e:u} ");
    }
    printf!("[{:u}]", *List::get(a, pivot).unwrap());
    for e in List::iter(r) {
        printf!(" {e:u}");
    }
}
