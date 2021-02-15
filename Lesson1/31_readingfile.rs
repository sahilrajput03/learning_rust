use std::fs;

fn main() {
  let contents = fs::read_to_string("testFile.txt").expect("Oops! Can not read the file...");

  println!("::File Contents => \n{}", contents);
}
// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
