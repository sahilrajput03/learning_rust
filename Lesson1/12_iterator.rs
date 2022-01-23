#[allow(dead_code)]
// MyLog: We are defining unsigned 8 bit integer. // Defining camelcase or othercase for binding name will throw warning, that you should use CAPITAL_CASE only.
// We can't use constants without defining data type for them, so if we remove type we'll get `error: missing type for `const` item`.;
const MY_VARIABLE: u8 = 3;
const MAXIMUM_NUMBER: u8 = 20;
fn main() {
	//We can use `const MAXIMUM_NUMBER: u8 = 20;` here too.
	for n in 1..MAXIMUM_NUMBER {
		println!("{}", n);
	}
	// MAXIMUM_NUMBER = 34 // We'll get `left-hand of expression is not valid` coz we can't re-initialize the value of constant.
}
