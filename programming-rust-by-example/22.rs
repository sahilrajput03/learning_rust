fn uppercase(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - 32,
        _ => c,
    }
}
fn main() {
    println!("we got: {}", uppercase(50));
}
