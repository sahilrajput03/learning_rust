#![allow(unused_mut)]

fn main() {
	// let my_vector: Vec<i32> = Vec::new(); // We are defining empty vector with i32 type here.
	// let mut my_vector = vec![10, 20, "50"];//! This would throw error coz vector expects same type of elements.
	// let mut my_vector: Vec<i32> = vec![10, 20, 30, 40]; //? WORKS GOOD:: Here we are giving types to vector elements.
	let mut my_vector = vec![10, 20, 30, 40]; // Here we infer types at compile time.

	println!("Element at index 2 in vector: {}\n", my_vector[2]); // prints value at index 2 i.e., '3'
	println!("");

	// my_vector.pop(); // Remove last element
	// my_vector.push(49); // Push element in the end
	// my_vector.remove(1); // Removes element @ index 1 i.e., second element.

	//? Printing the vector items: METHOD 1:
	for (index, a) in my_vector.iter().enumerate() {
		println!("The index is {} and the animal name is {}", index, a);
	}
	println!("");

	//? Printing the vector items: METHOD 2:
	for n in my_vector.iter() {
		println!("The animal name is {}", n);
	}
	println!("");

	//? Printing the vector items: METHOD 3:
	for n in &my_vector {
		//This^^^ is completely equivalent to above way of doing ```.iter()``` coz rust implements **reference to vector** as **into iter()**. ~Thansk to `Jon Gjengset` for this tip.
		println!("The animal name is {}", n);
	}
	println!("");

	//? Printing the vector items: METHOD 4:
	let mut i = 0;
	loop {
		if i == my_vector.len() {
			break;
		}

		println!("The index is {} and the animal name is {}", i, my_vector[i]);
		i += 1;
	}
	println!("");

	//? Printing the vector items: METHOD 5:
	println!("{:?}", my_vector);
	// LEARN: You have to use :? type caster for printing any vector.
}
