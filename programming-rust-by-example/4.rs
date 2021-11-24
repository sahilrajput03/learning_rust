fn main() {
	println!("{}", max(30, 40))
}

fn max(a: i32, b: i32) -> i32 {
	if a > b {
		a
	} else {
		b
	}
}

/*
The last expression in the body of a function is the value returned from the
function. You don't need to use return. The return keyword is only needed when
you want to return early.
*/
