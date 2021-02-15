fn main() {
  //We define iterator by using .. operator.
  for i in 1..11 {
    // We could have defned the range in a variable too. For e.g., let numbers = 30..51 and used that binding.
    //Learn: Upper bound is not incluside, so it ranges from 1 to 10.
    println!("The number is {}", i)
  }

  let range = 20..40;

  for i in range {
    println!("The number is {}", i)
  }
}
