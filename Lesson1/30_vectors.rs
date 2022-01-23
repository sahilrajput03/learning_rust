fn main() {
	// let my_vector: Vec<i32> = Vec::new();// One way to create a vector(but its a little difficult way though.)
	let mut my_vector = vec![10, 20, 30, 40];

	println!("{}\n", my_vector[2]); // prints value at index 2 i.e., '3'

	my_vector.push(49); // push at end of vector.

	my_vector.remove(1); // removes element @ index 1 i.e., '2'

	for n in my_vector.iter() {
		println!("{}", n);
	}

	println!("{:?}", my_vector);
	// LEARN: You have to use :? type caster for printing any vector.
}
