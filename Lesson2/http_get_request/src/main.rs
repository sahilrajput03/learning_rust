#[allow(dead_code)]
mod easy_way;
mod other;

#[allow(dead_code)]
fn test1() {
	easy_way::make_get_request()
		.map_err(|err| println!("WE GOT ERROR, SAHIL::::::: {:#?}", err)) //This is the way to get our error and result mapped, from stackoverflow.
		.ok(); // This helps remove warning*1.
}

#[allow(dead_code)]
fn test2() {
	// other::main(); // << Throws warning: //? warning: unused `std::result::Result` that must be used other::main()
	// other::main().ok(); // This fixes the warning.

	let result = other::main(); // This is another way to fix above warning.
	println!("I got resut as: {:?}", result); // ? Output: I got resut as: Ok(())
}

fn main() {
	// test1();
	test2();
}

//docs @ https://docs.rs/reqwest/0.11.0/reqwest/

// *1:
// = note: `#[warn(unused_must_use)]` on by default
// = note: this `Result` may be an `Err` variant, which should be handled
// src: https://stackoverflow.com/a/53368681
