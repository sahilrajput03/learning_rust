extern crate reqwest;
// This code works!
use error_chain::error_chain;
use std::io::Read; // This is not redundant!

error_chain! {// This is not redundant!
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
  let mut res = reqwest::blocking::get("https://reverberate.ml")?;
  let mut body = String::new();
  res.read_to_string(&mut body)?;

  println!("Status: {}", res.status());
  println!("Headers:\n{:#?}", res.headers());
  println!("Body:\n{}", body);
  // by default prints in good format!

  Ok(())
}
