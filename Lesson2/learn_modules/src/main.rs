#[allow(dead_code)]
mod dcode {
	// this is module definition.
	pub fn print_message() {
		println!("How's it going!");
		chicken(); //This tells us that we can call private fn's from public fns.
	}

	fn chicken() {
		println!("Chicken!");
	}

	pub mod water {
		pub fn print_message() {
			println!("I'm water!");
		}
	}
}

fn main() {
	// dcode::print_message(); // Works
	dcode::water::print_message(); // Works
	                           // BELOW STATMENT throws error coz functions are implicitly private in a module unless you specify them public with keyword ```pub```.
	                           // dcode::chicken(); // Throws error //! error[E0603]: function `chicken` is private
}
