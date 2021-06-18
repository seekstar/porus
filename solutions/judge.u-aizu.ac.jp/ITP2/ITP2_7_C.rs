fn main() {
    let mut q = default();
    scanf!("{:usize}", &mut q);
    let s = &mut BTreeSet::new();

    for _ in 0..q {
        let mut query = default();
        scanf!("{:u}", &mut query);

        match query {
            0 => {
                let mut x = default();
                scanf!("{:u}", &mut x);
                set::insert(s, x);
                printf!("{:usize}\n", collection::size(s));
            }
            1 => {
                let mut x = default();
                scanf!("{:u}", &mut x);
                printf!("{:u}\n", set::contains(s, &x));
            }
            2 => {
                let mut x = default();
                scanf!("{:u}", &mut x);
                set::remove(s, &x);
            }
            3 => {
                let (mut l, mut r) = default();
                scanf!("{:u} {:u}", &mut l, &mut r);
                for c in set::range(s, l..=r) {
                    printf!("{:u}\n", *c);
                }
            }
            _ => panic!("invalid query"),
        }
    }
}
