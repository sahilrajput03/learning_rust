fn main() {
	let p1 = Point { x: 11, y: 22 };
	print_point(&p1)
}

// References can also be used in the type of a function parameter. This is a
// function that prints a point, without moving the value:
fn print_point(point: &Point) {
	println!("x: {}, y: {}", point.x, point.y);
}

struct Point {
	x: i32,
	y: i32,
}
