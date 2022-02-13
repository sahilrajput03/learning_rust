// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

//? I AM NOT DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
	is_negative,
	is_zero,
}

impl PositiveNonzeroInteger {
	fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
		if value > 0 {
			Ok(PositiveNonzeroInteger(value as u64))
		} else if value == 0 {
			Err(CreationError::is_zero)
		} else {
			Err(CreationError::is_negative)
		}
	}
}

#[test]
fn test_creation() {
	assert!(PositiveNonzeroInteger::new(10).is_ok());
	assert_eq!(
		Err(CreationError::is_negative),
		PositiveNonzeroInteger::new(-10)
	);
	assert_eq!(Err(CreationError::is_zero), PositiveNonzeroInteger::new(0));
}
