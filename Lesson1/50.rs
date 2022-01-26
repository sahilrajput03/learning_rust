#[async_std::main]
async fn main()  {
	Ok(())
}

// Above code throws primary below error:
// ! #[async_std::main]
// ! |   ^^^^^^^^^ use of undeclared crate or module `async_std`
// ? So we must use async main function only with a cargo project only with ```async_std``` carte installed.
