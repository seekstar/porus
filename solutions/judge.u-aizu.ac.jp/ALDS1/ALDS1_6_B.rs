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

    list::iter(l).for_each(|e| printf!("{e:u} "));
    printf!("[{:u}]", *list::get(a, pivot));
    list::iter(r).for_each(|e| printf!(" {e:u}"));
}
