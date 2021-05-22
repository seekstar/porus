#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &Vec<u64> = &mut (0..n).map(|_| read!()).collect();

    interleave(list::iter(a).rev(), || printf!(" "), |e| printf!("{e:u}"));
    printf!("\n");
}
