extern crate rand;
use rand::Rng;

fn main() {
  let random_number = rand::thread_rng().gen_range(1, 11); // 1-10
  println!("Random Number: {}", random_number);

  //Flip a coin
  let random_bool = rand::thread_rng().gen_weighted_bool(2);
  // 2 in gen_weighted_bool() specifies that one in two is the probability
  // for true. Setting 2 is like flipping a coin.
  println!("Random Boolean: {}", random_bool);
}
