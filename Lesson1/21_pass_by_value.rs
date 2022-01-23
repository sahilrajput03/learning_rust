struct Color {
	red: u8, // u8: 0-255
	green: u8,
	blue: u8,
}

fn main() {
	let blue_color = Color {
		red: 0,
		green: 0,
		blue: 255,
	};

	print_color(blue_color);
}

fn print_color(c: Color) {
	println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}
