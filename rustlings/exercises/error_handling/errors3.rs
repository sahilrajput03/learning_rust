// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

//? I AM NOT DONE

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
	let mut tokens = 100;
	let pretend_user_input = "8";
	// let pretend_user_input = "8al"; // This is to simulate ParseIntError case of the program.

	let cost = total_cost(pretend_user_input)?;
	// LEARN: We added ParseIntError in the Resut of main function coz this total_cost function might
	// throw that error and we need to implement that on main function only.

	if cost > tokens {
		println!("You can't afford that many!");
		Ok(())
	// LEARN: Returning nothing means returning () but returning Ok(()) means returning Ok(())
	} else {
		tokens -= cost;
		println!("You now have {} tokens.", tokens);
		Ok(())
	}
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
	let processing_fee = 1;
	let cost_per_item = 5;
	let qty = item_quantity.parse::<i32>()?;

	// let qty = item_quantity.parse::<i32>()?;
	// ABOVE LINE THROWS ERROR:
	// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)

	Ok(qty * cost_per_item + processing_fee)
}
