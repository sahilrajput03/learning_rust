extern crate reqwest;
// This code works!
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
  let body = reqwest::blocking::get("https://reverberate.ml")?.text()?;

  println!("body = {}", body);
  // by default prints in good format!
  Ok(())
}
// what is reqwest::blocking  things??
// https://stackoverflow.com/a/58907538
