fn main() {
  let number = 10;

  match number {
    1 => println!("It is one!"),
    2 => println!("There is two of them!"),
    _ => println!("It doesn't match!"),
  }

  match number {
    1 => println!("It is one!"),
    2 => println!("There is two of them!"),
    10 | 11 => println!("It is either 10 or 11."),
    _ => println!("It doesn't match!"),
  }

  match number {
    1 => println!("It is one!"),
    2..=20 => println!("It is greater than 1."),
    _ => println!("It doesn't match!"),
  }

  let name = "Domenic";

  match name {
    "Chris" => println!("Nice name, mate!"),
    "Domenic" => println!("Yeah good on you, dcode!"),
    _ => println!("Don't know your name"),
  }
}

// reference for range inclusive/exclusive thing: https://github.com/rust-lang/rust/issues/37854#issuecomment-630503738
