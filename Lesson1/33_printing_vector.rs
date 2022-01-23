use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	println!("{:?}", args);
	// :? thats how we print a vector.
}
