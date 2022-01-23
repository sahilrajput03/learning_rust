fn main() {
	let animals = vec!["Rabbit", "Dog", "Cat"];

	for a in animals.iter() {
		println!("The animal name is {}", a);
	}
	println!("");
	//
	//

	// A BETTER WAY: Coz referencing to the vector gets us the ```iter()``` method. ~Sahil.
	for a in &animals {
		println!("The animal name is {}", a);
	}
}
