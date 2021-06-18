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
    for e in list::iter_mut(a) {
        scanf!(" {:c}{:u32}", &mut e.0, &mut e.1);
    }

    let bi: &mut Vec<usize> = &mut (0..n).collect();
    list::quick_sort(bi, |&i, &j| list::get(a, i).1 <= list::get(a, j).1);

    printf!(
        "{}\n",
        stable(list::is_stable_sort(a, |x, y| x.1 < y.1, bi))
    );
    for i in list::iter(bi) {
        printf!("{:c} {:u}\n", list::get(a, i).0, list::get(a, i).1);
    }
}
