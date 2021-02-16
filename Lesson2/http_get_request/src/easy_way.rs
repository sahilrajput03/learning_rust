extern crate reqwest;

use error_chain::error_chain;
use std::io::Read; // This is not redundant!

error_chain! {// This is not redundant!
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

const URL_TO_REQUEST: &str = "https://reverberate.ml/";
// const URL_TO_REQUEST: &str = "http://shittyurl123.com/"; // bad url test!

pub fn make_get_request() -> Result<()> {
  let mut res = reqwest::blocking::get(URL_TO_REQUEST)?;
  let mut body = String::new();
  res.read_to_string(&mut body)?;

  println!("Status: {}", res.status());
  println!("Headers:\n{:#?}", res.headers());
  println!("Body:\n{}", body);

  Ok(())
}
