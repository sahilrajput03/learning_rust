// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

//? I AM NOT DONE

fn main() {
	let mut x = 100;
	let y = &mut x;
	*y += 100; //? THIS LINE ROCKS HERE!
	let z = &mut x;
	// *y += 100; // Throws error //! second mutable borrow occurs here
	// LEARN: You can't mutatate ```x``` after the binding is mutably borrowed to some another bindig.
	*z += 1000;
	assert_eq!(x, 1200);
}
