// pg. 60; Constructors:
#[allow(dead_code)]
#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

#[allow(dead_code)]
impl Point {
	//? The difference with a normal method is that it does not take &self (or one of its variations) as a parameter.

	//? Rust does not provide constructors, but a common idiom is:
	//? static method (or say associated function)
	fn new(x: i32, y: i32) -> Self {
		Self { x: x, y: y }
		// Self {x, y}
		// ^^ that would be have been good as well bcoz :
		//? When the field name is the same as the value assigned(i.e, the parameter name which we used in the function ~sahil), it is possible to omit the value, as a shorthand as shown above.
	}
	//? Self is the type of the self value; we could have used Point instead of Self.

	fn origin() -> Self {
		Point { x: 0, y: 0 }
	}
}

fn main() {
	let p1 = Point::new(3, 4);
	//? Here ``new`` is a static method (what we usually call as constructor in other langs)
	println!("Point is: {:?}", p1);

	let p2 = Point::origin();
	println!("Origin point is: {:?}", p2)
}
