fn main() {
	let mut n = 0;

	loop {
		n += 1; // This is shorthand for n = n + 1
		if n == 10 {
			continue;
			// FYI: `continue` keyword says that the current loop iteration is surpassed and next iteration is continued.
		}

		println!("The value of n is {}", n);

		if n > 10 {
			break;
			// FYI: `break` keyword just says that loop iterations are stopped now.
		}
	}

	println!("\nProgram is completed, yikes!");
}
