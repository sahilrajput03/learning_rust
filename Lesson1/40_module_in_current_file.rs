#[allow(dead_code)]
mod house {
  pub const HOUSE_NUMBER: u32 = 56;
  pub mod bedroom1 {
    pub fn is_light_on() -> bool {
      false
    }

    pub fn is_neighbour_light_on() -> bool {
      use super::bedroom2;
      bedroom2::is_light_on()
    }
  }

  pub mod bedroom2 {
    pub fn is_light_on() -> bool {
      true
    }
  }
}

// use keyword: It brings modules to our current scope.
use house::bedroom1; // This works too.
                     // use house::{bedroom1, bedroom2}; //This is for importing multiple imports.

fn main() {
  println!("House no. {}", house::HOUSE_NUMBER);
  println!("Bedroom1 light: {}", house::bedroom1::is_light_on());
  println!("Bedroom2 light: {}", house::bedroom2::is_light_on());
  println!("\n");
  // after brining bedmoon1 to our current scope.
  println!("{}", bedroom1::is_light_on());
  println!("{}", bedroom1::is_neighbour_light_on());
}
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
// src: https://youtu.be/6cfcWzsvLrA
