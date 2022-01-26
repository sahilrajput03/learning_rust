fn main() {
	let tup1 = (101, 102, 103, 104, 105);
	// What is a tuple:
	// ? Tuple types are a family of structural types1 for heterogeneous lists of other types.
	// Src: https://doc.rust-lang.org/reference/types/tuple.html#tuple-types

	println!("{}", tup1.2);
	// println!("{}", tup1[2]); //LEARN: Throws error!!
	// prints 30, i.e., value @ index 2.
	println!("\n");

	// LEARN: Access a element in a nested tuple.
	let tup2 = (11, (100, 200, 300), "rust", 3.4, false);
	// Nested tuple ↑↑↑ inside tuple.
	println!("{}", (tup2.1).2);
	println!("\n");

	// LEARN: Destructuring tuple elements in rust.
	let tup3 = (45, 6.7, "Some text.");
	let (a, b, c) = tup3;
	println!("a is {}", a); // prints `a is 30`
	println!("b is {}", b); // prints `b is 7`
	println!("c is {}", c); // prints `c is Some text.`
}
