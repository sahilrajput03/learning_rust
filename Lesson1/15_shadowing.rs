// Shadoing and redeclaration nature of rust
#[allow(unused_variables, unused_parens, dead_code, unused_mut)] //This is also legit too.

fn main() {
	let mut x = 10;
	println!("x: {}", x);
	{
		println!("x inside block scope: {}", x);
		let x = 15; // This represents shadowing.
		println!("==>Shadowed value of x: {}", x);
	}

	let x = "x_text";
	println!("==>Re-assigned string value to x: {}", x);

	let x = true;
	println!("==>Re-assigned boolean value to x: {}", x);
}
