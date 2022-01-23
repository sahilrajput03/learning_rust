struct Person {
	name: String,
	age: u8,
}

// Define your trait.
trait HasVoiceBox {
	//Speak
	fn speak(&self);
	// Check if can speak.
	fn can_speak(&self) -> bool;
}

// Implement your trait.
impl HasVoiceBox for Person {
	fn speak(&self) {
		println!("Hello, my name is {}", self.name);
	}

	fn can_speak(&self) -> bool {
		if self.age > 0 {
			return true;
		}
		return false;
	}
}

fn main() {
	let person = Person {
		name: String::from("Bob"),
		age: 1,
		// try chaning value to either 0 or some positive no. for program demo.
	};
	println!("Can {} speak? {}", person.name, person.can_speak());
}
