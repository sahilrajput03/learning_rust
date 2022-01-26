#![allow(unused_variables, dead_code)]

fn main() {
	println!("{}", max(30, 40));
	println!("Returning string from function: {}", get_string());
	println!("3 is divisible by 2: {}", is_divisible_by(3, 2));
	println!("{}", to_english(1));
	println!("{}", to_english(2));
	println!("{}", to_english(4));

	// Another way of returning string is via using reference of string: &str
	println!("{}", long_string("carl is"));
	println!("{}", long_string("carl is here"));
	println!("{}", long_string1());
}

//? RETURNING INTEGER FROM FUNCITON:
fn max(a: i32, b: i32) -> i32 {
	if a > b {
		a
	} else {
		b
	}
}
/*
The last expression in the body of a function is the value returned from the
function. You don't need to use return. The return keyword is only needed when
you want to return early.
*/

//? RETURNING STRING FROM A FUNCTION:
fn get_string() -> String {
	"Hello wordl!".to_string()
	// Note .to_string() is important else compiler throws error: //! help: try using a conversion method: `.to_string()`   \n  expected struct `String`, found `&str`.
}

//? RETURNING BOOLEAN FROM FUNCTION:
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
	// Corner case, early return
	if rhs == 0 {
		return false;
	}

	// This is an expression, the `return` keyword is not necessary here
	lhs % rhs == 0
}

fn to_english(i: i32) -> String {
	match i {
		1 => {
			println!("Case 1 executing..");
			"Number is one.".to_string() // This is returned from function coz there's no semicolon in the end of line.
		}
		2 => "Number is two.".to_string(),
		_ => "Number is other than one or two.".to_string(),
	}
}

// Difference b/w String and str in rust? : https://stackoverflow.com/a/24159874/10012446
// But what exactly is &str ?: https://stackoverflow.com/a/24159933/10012446
fn long_string(x: &str) -> &str {
	if x.len() > 10 {
		"too long"
	} else {
		x
	}
}

// fn long_string1() -> &str {// that throws error like below:
/* //
! expected named lifetime parameter
! help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
! help: consider using the `'static` lifetime; fn long_string1() -> &'static str
*/
fn long_string1() -> &'static str {
	if true {
		"too long"
	} else {
		"not too long"
	}
}

// Read awesome text about lifetimes in rust: ~sahil.
// https://stackoverflow.com/a/43080280/10012446
// https://stackoverflow.com/a/69598116/10012446
