#[derive(Default, Clone, Copy)]
struct Card(u8, u8);

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
        scanf!(" {:c}{:u8}", &mut e.0, &mut e.1);
    }

    let bi: &mut Vec<usize> = &mut (0..n).collect();
    List::bubble_sort(bi, |&i, &j| {
        List::get(a, i).unwrap().1 < List::get(a, j).unwrap().1
    });
    interleave(
        List::iter(bi),
        || printf!(" "),
        |i| {
            printf!(
                "{:c}{:u}",
                List::get(a, i).unwrap().0,
                List::get(a, i).unwrap().1
            )
        },
    );
    printf!(
        "\n{}\n",
        stable(List::is_stable_sort(a, |x, y| x.1 < y.1, bi))
    );

    let si: &mut Vec<usize> = &mut (0..n).collect();
    List::selection_sort(si, |&i, &j| {
        List::get(a, i).unwrap().1 < List::get(a, j).unwrap().1
    });
    interleave(
        List::iter(si),
        || printf!(" "),
        |i| {
            printf!(
                "{:c}{:u}",
                List::get(a, i).unwrap().0,
                List::get(a, i).unwrap().1
            )
        },
    );
    printf!(
        "\n{}\n",
        stable(List::is_stable_sort(a, |x, y| x.1 < y.1, si))
    );
}
