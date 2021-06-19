#[derive(Default, Clone, Copy)]
struct Card(u8, u32);

fn stable(b: bool) -> &'static str {
    if b {
        "Stable"
    } else {
        "Not stable"
    }
}

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let a = &mut vec![Card(0, 0); n];
    for e in List::iter_mut(a) {
        scanf!(" {:c}{:u32}", &mut e.0, &mut e.1);
    }

    let bi: &mut Vec<usize> = &mut (0..n).collect();
    List::quick_sort(bi, |&i, &j| {
        List::get(a, i).unwrap().1 <= List::get(a, j).unwrap().1
    });

    printf!(
        "{}\n",
        stable(List::is_stable_sort(a, |x, y| x.1 < y.1, bi))
    );
    for i in List::iter(bi) {
        printf!(
            "{:c} {:u}\n",
            List::get(a, i).unwrap().0,
            List::get(a, i).unwrap().1
        );
    }
}
