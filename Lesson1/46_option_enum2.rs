// You'll learn about `Option` keyword in this example!
// src: https://youtu.be/JKmkKae-EhM
// @docs: https://doc.rust-lang.org/std/option/enum.Option.html
fn main() {
	let occupation = get_occupation("Domenic");

	println!(
		"Occupation is {}",
		match occupation {
			Some(o) => o,
			None => "not found.",
		}
	);
}

fn get_occupation(name: &str) -> Option<&str> {
	match name {
		"Domenic" => Some("Software Developer."),
		"Michael" => Some("Dentist."),
		_ => None,
	}
}

// FYI: We could have called the get_occupation function in the match expression as well if we wanted:alloc
// I.e.,
// match get_occupation("Domenic") {
// match get_occupation("Michael") {
// match get_occupation("Sahil") {
