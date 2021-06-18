fn main() {
    let (mut a, mut b) = default();
    scanf!("{:u} {:u}", &mut a, &mut b);
    let area = a * b;
    let perimeter = (a + b) * 2;
    printf!("{area:u} {perimeter:u}\n");
}
