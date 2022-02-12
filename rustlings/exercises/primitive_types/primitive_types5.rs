// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

//? I AM NOT DONE

fn main() {
	let cat = ("Furry McFurson", 3.5);
	// LEARN: Tuple types is a ```list of hetegeroneous types```.
	// LEARN: Destructuring is super cool in rust.
	let (name, age)/* your pattern here */ = cat;

	println!("{} is {} years old.", name, age);
}
