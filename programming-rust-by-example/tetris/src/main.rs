// pg. 126
use std::fs::File; // File is a type.
use std::io::{self, Write};
// we import the io module (self) and the Write trait.

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}
fn main() {
    println!("hello")
}
