#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let (mut a, mut b): (isize, isize) = default();
    scanf!(stdin, " %d %d", &mut a, &mut b);
    printf!(stdout,
            "a %s b\n",
            match Ord::cmp(&a, &b) {
                Less => "<",
                Equal => "==",
                Greater => ">",
            });
}
