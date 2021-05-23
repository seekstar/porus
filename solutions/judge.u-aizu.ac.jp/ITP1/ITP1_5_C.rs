#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let (mut h, mut w) = default();
        scanf!("{:u} {:u}", &mut h, &mut w);
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
