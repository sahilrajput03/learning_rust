pub fn is_light_on() -> bool {
  false
}

pub fn is_neighbour_light_on() -> bool {
  use super::medroom2;
  medroom2::is_light_on()
}
