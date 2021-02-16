#[allow(dead_code)]
mod easy_way;

fn main() {
  easy_way::make_get_request()
    // .map_err(|err| println!("{:?}", err)) //This is the way to get our error and result mapped from stackoverflow.
    .ok(); // This helps remove warning*1.
}

//docs @ https://docs.rs/reqwest/0.11.0/reqwest/

// *1:
// = note: `#[warn(unused_must_use)]` on by default
// = note: this `Result` may be an `Err` variant, which should be handled
// src: https://stackoverflow.com/a/53368681
