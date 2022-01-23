fn uppercase(c: u8) -> u8 {
	// ^ returns the acii of its uppercase
	match c {
		b'a'..=b'z' => c - 32,
		_ => c,
	}
}
fn main() {
	let k = 97; // 97 is ascii for a
	println!("binary of A: {}", b'A');
	// 1000001 in binary

	println!("binary of a: {}", b'a');
	// 1100001 in binary

	println!("");
	println!("");
	println!("we got: {}", uppercase(k));
	println!("we got2: {}", uppercase(k) as char);
}

// below function is a rewrite of above using if-else approach.
// fn uppercase(c: u8) -> u8 {
//     if let b'a'...b'z' = c {
//         c - 32
//     } else {
//         c
//     }
// }
