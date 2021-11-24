fn main() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  // We are specifying ↑ [datatypeOfElement; lengthOfArray]

  for n in numbers.iter() {
    println!("way1: {}", n)
  }

  println!("\n");

  for n in 0..numbers.len() {
    println!("way2: {}", numbers[n])
  }
}
