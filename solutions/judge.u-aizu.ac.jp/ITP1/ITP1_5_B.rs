fn main() {
    loop {
        let (mut h, mut w) = default();
        scanf!("{:u} {:u}", &mut h, &mut w);
        if (h == 0) && (w == 0) {
            break;
        }

        for _ in 0..w {
            printf!("#");
        }
        printf!("\n");

        for _ in 0..(h - 2) {
            printf!("#");
            for _ in 0..(w - 2) {
                printf!(".");
            }
            printf!("#\n");
        }

        for _ in 0..w {
            printf!("#");
        }
        printf!("\n");

        printf!("\n");
    }
}
