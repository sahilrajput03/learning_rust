use error_chain::error_chain;
use std::io::Read; // This is not redundant!
error_chain! {// This is not redundant!
	foreign_links {
		Io(std::io::Error);
		HttpRequest(reqwest::Error);
	}
}

const URL_TO_REQUEST: &str = "https://sahilrajput03.github.io/";
// const URL_TO_REQUEST: &str = "http://shittyurl123.com/"; // bad url test!
// ? LEARN: When you use a bad url the program would compile but //! stops executiong at that statement.
pub fn make_get_request() -> Result<()> {
	let mut res = reqwest::blocking::get(URL_TO_REQUEST)?;
	let mut body = String::new();
	res.read_to_string(&mut body)?;

	println!("WOW: Status: {}", res.status());
	println!("WOW: Headers:\n{:#?}", res.headers());
	// println!("WOW: Body:\n{}", body); //? LEARN: This prints the response html to stdoutput.

	Ok(())
}
