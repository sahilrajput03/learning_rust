use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	let mut file = File::create("outputTest.txt")?;
	file.write_all(b"Hello, world!")?;
	Ok(())
}
// src: https://doc.rust-lang.org/std/fs/struct.File.html
