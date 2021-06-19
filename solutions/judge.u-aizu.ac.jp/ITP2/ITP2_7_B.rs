fn main() {
    let mut q = default();
    scanf!("{:usize}", &mut q);
    let s = &mut BTreeSet::new();

    for _ in 0..q {
        let (mut query, mut x) = default();
        scanf!("{:u} {:u}", &mut query, &mut x);

        match query {
            0 => {
                set::insert(s, x);
                printf!("{:usize}\n", Collection::size(s));
            }
            1 => {
                printf!("{:u}\n", set::contains(s, &x));
            }
            2 => {
                set::remove(s, &x);
            }
            _ => panic!("invalid query"),
        }
    }
}
