fn main() {
    let mut x = default();
    scanf!("{:u}", &mut x);
    let cube = x * x * x;
    printf!("{cube:u}\n");
}
