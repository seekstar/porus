#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut t: isize = default();
    scanf!(stdin, "%d", &mut t);
    let s = t % 60;
    let mut m = t / 60;
    let h = m / 60;
    m = m % 60;
    printf!(stdout, "%d:%d:%d\n", h, m, s);
}
