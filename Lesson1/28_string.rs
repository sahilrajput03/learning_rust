fn main() {
	let mut my_string = String::from("How's is going? My name is Sahil.");

	println!("length: {}", my_string.len());
	println!("String is empty?: {}", my_string.is_empty());
	// AMAZING strings methods from article: https://www.tutorialspoint.com/rust/rust_string.htm#:~:text=The%20split()%20string%20method,split()%20as%20a%20vector.
	// MUST READ AND DO REPLICATE ALL THESE HERE>^^^

	for token in my_string.split_whitespace() {
		println!("{}", token);
	}

	println!("\n");

	println!(
		"Does the string contains 'Sahil'? {}",
		my_string.contains("Sahil")
	);

	my_string.push_str(" Welcome to the string tutorials!");

	println!("String is now: {}", my_string);
}
