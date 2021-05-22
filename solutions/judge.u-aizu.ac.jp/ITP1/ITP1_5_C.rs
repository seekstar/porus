#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let h: u64 = read!();
        let w: u64 = read!();
        if (h == 0) && (w == 0) {
            break;
        }

        for i in 0..h {
            for j in 0..w {
                printf!(
                    "{}",
                    match (i % 2) == (j % 2) {
                        false => ".",
                        true => "#",
                    }
                );
            }
            printf!("\n");
        }
        printf!("\n");
    }
}
