#[allow(dead_code)]

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn main() {
  let player_direction: Direction = Direction::Down;

  match player_direction {
    Direction::Up => println!("We are heading up"),
    Direction::Down => println!("We are heading down"),
    Direction::Left => println!("We are heading left"),
    Direction::Right => println!("We are heading right"),
    // Using None case throws error, coz we defined the case value's *type* in the first line using match keyword too.
    // None => println!("No direction, LOLS"),
  }
}
