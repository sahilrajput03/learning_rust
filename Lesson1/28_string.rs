fn main() {
	let mut my_string = String::from("How's is going? My name is Sahil.");

	println!("length: {}", my_string.len());
	println!("String is empty?: {}", my_string.is_empty());

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
