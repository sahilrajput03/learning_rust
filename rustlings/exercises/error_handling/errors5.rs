// errors5.rs

// This program uses a completed version of the code from errors4.
// It won't compile right now! Why?
// Execute `rustlings hint errors5` for hints!

//? I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
// LEARN_IMP: Instead of doing something like => ```Result<(), ParseIntError | CreationError>```.
// WE DO LIKE boxing of errors but it remains vulnerable to runtime errors thrown though as stated
// PLEASE READ ABOUT Boxing @ https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html
fn main() -> Result<(), Box<dyn error::Error>> {
	let pretend_user_input = "42";
	let x: i64 = pretend_user_input.parse()?;
	println!("output={:?}", PositiveNonzeroInteger::new(x)?);
	Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
	Negative,
	Zero,
}

impl PositiveNonzeroInteger {
	fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
		match value {
			x if x < 0 => Err(CreationError::Negative),
			x if x == 0 => Err(CreationError::Zero),
			x => Ok(PositiveNonzeroInteger(x as u64)),
		}
	}
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let description = match *self {
			CreationError::Negative => "number is negative",
			CreationError::Zero => "number is zero",
		};
		f.write_str(description)
	}
}

impl error::Error for CreationError {}
