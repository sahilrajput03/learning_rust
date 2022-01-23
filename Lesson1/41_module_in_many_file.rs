mod house;

// use keyword: It brings modules to our current scope.
use house::bedroom1; // This works too.
					 // use house::{bedroom1, bedroom2}; //This is for importing multiple imports.

fn main() {
	println!("House no. {}", house::HOUSE_NUMBER);
	println!("Bedroom1 light: {}", house::bedroom1::is_light_on());
	println!("Bedroom2 light: {}", house::bedroom2::is_light_on());
	println!("\n");
	// after brining bedmoon1 to our current scope.
	println!("{}", bedroom1::is_light_on());
	println!("{}", bedroom1::is_neighbour_light_on());
}
// src: https://youtu.be/6cfcWzsvLrA
