fn main() {
	assert_eq!(Some(41), Some(42)); // assert_eq macro is for testing two expressions during debugging, src: https://doc.rust-lang.org/core/macro.assert_eq.html
								// ! ^^ this makes the rust compiler panic coz ```first expression``` is not equal to ```second expression```.
	println!("Hello!");
}
