fn main() {
	is_even_or_odd(6);
}

fn is_even_or_odd(num: u32) {
	for n in 1..num {
		if is_even(n) {
			println!("{} is even.", n);
		} else {
			println!("{} is odd.", n)
		}
	}
}

fn is_even(num: u32) -> bool {
	return num % 2 == 0;
}

//Output:
//1 is odd.
//2 is even.
//3 is odd.
//4 is even.
//5 is odd.
