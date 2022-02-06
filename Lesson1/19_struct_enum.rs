#![allow(dead_code, unused)]

fn array_like_structs() {
	struct Color(u8, u8, u8); //? tuple/array like struct with parentheses.

	let mut amz = Color(255, 0, 0);
	amz.1 = 20;
	println!("amz is {}, {}, {}", amz.0, amz.1, amz.2);

	//? LEARN:  Destructured the instance using a `let` binding, this will not destruct black instance
	let Color(red, green, blue) = amz;
	println!("amz (destructured way) is {}, {}, {}", red, green, blue);
}

fn c_like_structs() {
	struct Color {
		red: u8,   //? C like struct with braces.
		green: u8, //? u8: 0-255
		blue: u8,
	}

	let mut wow = Color {
		red: 20,
		green: 30,
		blue: 49,
	};
	println!("wow : {}, {}, {}", wow.red, wow.green, wow.blue);

	//? LEARN: Destructure the instance using a `let` binding, this will not destruct blue instance
	let Color {
		red,
		green: g, // Renamed after destructuring, SAME AS JAVASCRIPT.
		blue: b,  // Renamed after destructuring, SAME AS JAVASCRIPT.
	} = wow;
	println!("wow (destructured way) : {}, {}, {}", red, g, b);
}

fn main() {
	array_like_structs();
	println!(""); // Empty line to separate output of fn1 and c_like_structs.
	c_like_structs();
}
