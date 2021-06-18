fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![default(); n];
    for e in list::iter_mut(a) {
        scanf!("{:u}", e);
    }
    interleave(list::iter(a).rev(), || printf!(" "), |e| printf!("{e:u}"));
    printf!("\n");
}
