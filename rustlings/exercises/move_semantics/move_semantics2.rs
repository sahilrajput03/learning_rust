// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

//? I AM NOT DONE
// ! THIS IS A SUPER AMAZING EXERCISE TO PRACTISE EVERYTHING ABOUT MOVE IN RUST.

// ~MY LEARNINGS:
// 1. We can move value to a function.
// 2. We can borrow using & and then use .clone method to clone the binding into another and then we can mutate that binding freely.
// 3. We can borrow mutably and freely mutate that binding
fn main() {
	// solutn way 1 and solutn way 2
	let vec0: Vec<i32> = Vec::new();

	// solutn way 1
	// let vec2 = Vec::new();

	// solutn way 1
	// let mut vec1 = fill_vec(vec2);

	// solutn way 3
	// let mut vec0: Vec<i32> = Vec::new();

	// solutn way 2 (i.e., borrow and clone to get ownership in different binding)
	let mut vec1 = fill_vec(&vec0);

	// solutn way 3 (i.e., mutably borrow)
	// let mut vec1 = fill_vec(&mut vec0);

	// Do not change the following line!
	println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

	vec1.push(88);
	println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// solutn way 1
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
// solutn way 2
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
	// solutn way 3
	// fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
	// solutn way 1 and solutn way 3
	// let mut vec = vec;

	// solutn way 2 (this way we can alter the values which are simply borrowed (i.e., non-mutable borrow)).
	let mut vec = vec.clone();

	vec.push(22);
	vec.push(44);
	vec.push(66);

	vec.to_vec()
}
