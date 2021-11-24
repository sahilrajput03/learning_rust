struct Person {
  name: String,
  age: u8,
}

impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("My name is {} and I am {}.", self.name, self.age);
  }
}

fn main() {
  let dom = Person {
    name: String::from("Sahil Rajput"),
    age: 25,
  };
  println!("{}", dom.to_string());
}
