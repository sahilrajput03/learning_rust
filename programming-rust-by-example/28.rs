fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
fn main() {
    println!("Hello");
    println!("{}", max('a', 'z'));
}
