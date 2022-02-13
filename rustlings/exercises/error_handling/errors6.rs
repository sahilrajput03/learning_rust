// errors6.rs

// LEARN: BELOW TEXT IS FROM RUSTLINGS ITSELF.
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)

//? I AM NOT DONE

// ~sahil:: HINT FOR THIS CASE:

// This exercise uses a completed version of `PositiveNonzeroInteger` from
// errors4.

// Below the line that TODO asks you to change, there is an example of using
// the `map_err()` method on a `Result` to transform one type of error into
// another. Try using something similar on the `Result` from `parse()`. You
// might use the `?` operator to return early from the function, or you might
// use a `match` expression, or maybe there's another way!

// You can create another function inside `impl ParsePosNonzeroError` to use
// with `map_err()`.

// Read more about `map_err()` in the `std::result` documentation:
// https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
	Creation(CreationError),
	ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
	fn from_creation(err: CreationError) -> ParsePosNonzeroError {
		ParsePosNonzeroError::Creation(err) // This refers to zero or -ve case.
	}

	// TODO: add another error conversion function here.
	fn from_parse_int(err: ParseIntError) -> ParsePosNonzeroError {
		ParsePosNonzeroError::ParseInt(err) // This refers to non parseable integer case.
	}
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
	// TODO: change this to return an appropriate error instead of panicking
	// when `parse()` returns an error.
	// let x: i64 = s.parse().unwrap();// ORIGINALLY GIVEN.

	// LEARN: ? is like bull in the china market coz when it find Err, it just kill the program at
	// runtime.
	// SRC: https://github.com/rust-lang/rustlings/issues/808
	// SRC: https://docs1.w3cub.com/rust/book/error-handling/
	// LEARN: ? mark operator is used to early return from a block of code or a function.

	let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
	PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}
// FOR TEST CASES TO PASS: required errors for zero and -ve cases.
// Err(ParsePosNonzeroError::ParseInt(_)) // For not a number!
// Err(ParsePosNonzeroError::Creation(CreationError::Negative))
// Err(ParsePosNonzeroError::Creation(CreationError::Zero))

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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse_error() {
		// We can't construct a ParseIntError, so we have to pattern match.
		assert!(matches!(
			parse_pos_nonzero("not a number"),
			Err(ParsePosNonzeroError::ParseInt(_))
		));
	}

	#[test]
	fn test_negative() {
		assert_eq!(
			parse_pos_nonzero("-555"),
			Err(ParsePosNonzeroError::Creation(CreationError::Negative))
		);
	}

	#[test]
	fn test_zero() {
		assert_eq!(
			parse_pos_nonzero("0"),
			Err(ParsePosNonzeroError::Creation(CreationError::Zero))
		);
	}

	#[test]
	fn test_positive() {
		let x = PositiveNonzeroInteger::new(42);
		assert!(x.is_ok());
		assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
	}
}
