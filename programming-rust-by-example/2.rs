// Writing condition
/*
Pg. 29
One particularity of Rust conditions, like many other constructs, is that they are
expressions. The last expression of each branch is the value of this branch. Be
careful though, the type of each branch must be the same. For instance, we can
get the minimum number of the two numbers and put it into a variable:
 */

fn main() {
	let num1 = 25;
	let num2 = 42;

	let minimum = if num1 < num2 { num1 } else { num2 };

	println!("{} is minimum value.", minimum);
}
