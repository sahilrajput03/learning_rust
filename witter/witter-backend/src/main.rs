#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))] // ^^ all this is to disable unused import warnings.
#[macro_use]
extern crate dotenv_codegen;

use std::env;

fn main() {
	println!("{}", dotenv!("DATABASE_URL"));
	println!("Hello, world 3");
}
// continue from:: https://youtu.be/yNe9Xr35n4Q?t=2113
