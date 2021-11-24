fn main() {
  let numbers = [2; 10];
  // Making array [value of all elements; with size]
  for n in numbers.iter() {
    println!("way1: {}", n)
  }

  println!("\n");

  for n in 0..numbers.len() {
    println!("way2: {}", numbers[n])
  }
}
