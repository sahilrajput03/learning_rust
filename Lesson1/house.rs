// This file represents house module, yikes!
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
