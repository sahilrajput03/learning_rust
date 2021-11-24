fn main() {
  let name = String::from("Domenic");

  println!(
    "Character at index 8: {}",
    match name.chars().nth(6) {
      Some(c) => c.to_string(),
      None => "No character at index 8!".to_string(),
    }
  );
  // FYI: name.chars().nth(8) return either `Some` or `None` case.
}
