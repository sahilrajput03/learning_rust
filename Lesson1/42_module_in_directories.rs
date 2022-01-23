mod mouse; // This name of directory though.

// use keyword: It brings modules to our current scope.
use mouse::medroom1; // This works too.
					 // use mouse::{medroom1, medroom2}; //This is for importing multiple imports.

fn main() {
	println!("Mouse no. {}", mouse::MOUSE_NUMBER);
	println!("Medroom1 light: {}", mouse::medroom1::is_light_on());
	println!("Medroom2 light: {}", mouse::medroom2::is_light_on());
	println!("\n");
	// after brining bedmoon1 to our current scope.
	println!("{}", medroom1::is_light_on());
	println!("{}", medroom1::is_neighbour_light_on());
}
// src: https://youtu.be/6cfcWzsvLrA
