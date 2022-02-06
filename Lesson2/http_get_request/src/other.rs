use error_chain::error_chain;
use http::StatusCode;
use std::io::Read;

error_chain! {
	foreign_links {
		Io(std::io::Error);
		HttpRequest(reqwest::Error);
	}
}

#[allow(dead_code)]
pub fn funny() -> Result<()> {
	let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
	let mut body = String::new();
	res.read_to_string(&mut body)?;

	println!("Status: {}", res.status());
	println!("Headers:\n{:#?}", res.headers());
	println!("Body:\n{}", body);

	Ok(())
}

#[allow(dead_code)]
pub fn fun1() {
	// other::funny(); // << Throws warning: //? warning: unused `std::result::Result` that must be used other::funny()
	// other::funny().ok(); // This fixes the warning.

	let result = funny(); // This is another way to fix above warning.
	println!("I got resut as: {:?}", result); // ? Output: I got resut as: Ok(())
}


#[allow(dead_code, unused_mut, unused_variables)]
pub async fn fun2() -> Result<()> {
	println!("FUNCTION main2 is executing now...");
	match reqwest::get("https://sahilrajput03.github.io").await {
		Ok(mut response) => {
			println!("Reached here.");
			// check if 200 OK
			if response.status() == StatusCode::OK {
				match response.text().await {
					Ok(text) => {
						println!("We got the response successfully #286");
						// println!("We got the response #286: {}", text)

					},
					Err(err) => println!("Error:1: {}", err),
				}
			} else {
				println!("Response was not 200 OK.");
			}
		}
		Err(err) => println!("Error:2: {}", err),
	};
	Ok(())
}

