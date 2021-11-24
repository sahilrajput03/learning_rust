fn main() {
  let animals = vec!["Rabbit", "Dog", "Cat"];

  for (index, a) in animals.iter().enumerate() {
    println!("The index is {} and the animal name is {}", index, a);
  }
}
