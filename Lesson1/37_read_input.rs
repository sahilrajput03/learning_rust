#[allow(unused_mut)]
use std::io;

fn main() {
	let mut input = String::new();

	println!("Hey mate! Say something:");

	match io::stdin().read_line(&mut input) {
		Ok(_) => {
			println!("Success! You said: {}", input.to_uppercase());
			// input is actually a string. Yikes!
		}
		Err(e) => println!("Oops! Something went wrong: {}", e),
	}
}
