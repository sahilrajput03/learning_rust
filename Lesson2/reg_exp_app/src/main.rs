extern crate regex;
use regex::Regex;

fn main() {
  let re = Regex::new(r"\w{5}").unwrap();
  // TIP: ↑↑↑ This matches for any 5 letter word.
  let text = "dcode"; // Try deleting any char from the word and test it...

  println!("Found match? {}", re.is_match(text));

  let re2 = Regex::new(r"(\w{5})").unwrap();
  match re2.captures(text) {
    Some(caps) => println!("Found match: {}", &caps[0]), // Below one also works but is a little complicated to remember.
    // Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
    None => println!("Could not find match..."),
  }
}

// src: https://youtu.be/B7koBE7VDGo
