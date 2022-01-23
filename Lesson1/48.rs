#[allow(unreachable_code)]

fn main() {
	{
		return;
		println!("Hello world!");
		// Only `string literal` can be passed to println! macro.
		//LEARN: In `println!` ! means the execution of macro, yes println is a macro, and we need ! to call it.
		let x = 45;
		println!("The value of x is {}", x);
	}
	{
		return; // Intentionally not executing below code using return statement here. ~Sahil.
		let x = 45;
		println!("The value of x is {}", x);
	}
}
