fn is_alphanumeric(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false,
    }
}

fn main() {
    println!("is b alphanumeric: {}", is_alphanumeric('b'));
    println!("is @ alphanumeric: {}", is_alphanumeric('@'));
}
