#[allow(dead_code)]
mod easy_way;
mod other;
use async_std::task::block_on; //? IMPORTANT: async_std only seems to work via adding features from it ```async-std = { version = "1", features = ["attributes", "tokio1"] }```, source: https://stackoverflow.com/a/68401216/10012446

// ? MUST READ ASYNC PROGRAMMING IN RUST - OFFICIAL BOOK - https://rust-lang.github.io/async-book/

#[allow(unused_must_use)]
fn main() {
	// other::fun1();
	block_on(other::fun2());
	// easy_way::main();
}

//docs @ https://docs.rs/reqwest/0.11.0/reqwest/

// *1:
// = note: `#[warn(unused_must_use)]` on by default
// = note: this `Result` may be an `Err` variant, which should be handled
// src: https://stackoverflow.com/a/53368681
