#![cfg_attr(debug_assertions, allow(dead_code))]
#[allow(unreachable_code, dead_code)]
use std::io;

// src: https://doc.rust-lang.org/reference/attributes/testing.html
// Note: Note: The test mode is enabled by passing the --test argument to rustc or using cargo test. ~ Source: <same above official link>
//? FYI: Sahil: Use your watch script executable in the root of this repository i.e., ```rmonTestSuite.sh``` to run this file.

#[test]
fn test_the_thing() -> io::Result<()> {
	// let state = setup_the_thing()?; // expected to succeed
	// do_the_thing(&state)?;          // expected to succeed
	Ok(())
}

#[test]
fn test_another() -> io::Result<()> {
	// do some stuff here....
	Ok(())
}

#[ignore = "not yet implemented"]
fn mytest() {
	// …
}

/*

Output:
	running 2 tests
	test test_another ... ok
	test test_the_thing ... ok

	test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/
