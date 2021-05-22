#![feature(exclusive_range_pattern)]

#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let m: i64 = read!();
        let f: i64 = read!();
        let r: i64 = read!();

        if (m == -1) && (f == -1) && (r == -1) {
            break;
        }

        if (m == -1) || (f == -1) {
            printf!("F\n");
        } else {
            printf!(
                "{}\n",
                match m + f {
                    80..=100 => "A",
                    65..80 => "B",
                    50..65 => "C",
                    30..50 if r >= 50 => "C",
                    30..50 => "D",
                    0..30 => "F",
                    _ => panic!(),
                }
            );
        }
    }
}
