fn main() {
	/*Replace*/
	{
		let my_string = String::from("Rust is fantastic!");
		println!("After replace: {}", my_string.replace("fantastic", "great"));
	}

	/*Lines*/
	{
		let my_string = String::from("The weather is\nnice\noutside mate!");

		for line in my_string.lines() {
			println!("[{}]", line);
		}
	}

	/*Split*/
	{
		let my_string = String::from("Leave+a+like+if+you+enjoyed!");
		let tokens: Vec<&str> = my_string.split("+").collect();

		println!("At index 2: {}", tokens[2]);
	}

	{
		let my_string = String::from("      My name is Domenic   \n\r");

		println!("Before trim: {}", my_string);
		println!("After trim: {}", my_string.trim());
	}

	/*Chars*/
	{
		let my_string = String::from("dcode on YouTube");

		println!("my_string has char@index 4: {:?}", my_string.chars().nth(4));
		// {} is deafult formatter
		// {:?} to format to string
		// {:#?} to pretty-print

		/*Get character at index */
		println!("\n");
		match my_string.chars().nth(4) {
			Some(c) => println!("Character at index 4: {}", c),
			None => println!("No character at index 4."),
		}
	}
}
