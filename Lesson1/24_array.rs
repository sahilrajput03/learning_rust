fn main() {
	let numbers = [1, 2, 3, 4, 5];
	// numbers[0] //Ouput: 1
	// numbers[1] //Ouput: 2

	for n in numbers.iter() {
		println!("way1: {}", n)
	}

	println!("\n");

	for n in 0..numbers.len() {
		println!("way2: {}", numbers[n])
	}
}
