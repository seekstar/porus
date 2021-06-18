fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in list::iter_mut(a) {
        scanf!("{:u}", e);
    }

    let pivot = sorting::partition(a, PartialOrd::le);

    let l = &list::slice(a, ..pivot);
    let r = &list::slice(a, (pivot + 1)..);

    for e in list::iter(l) {
        printf!("{e:u} ");
    }
    printf!("[{:u}]", *list::get(a, pivot));
    for e in list::iter(r) {
        printf!(" {e:u}");
    }
}
