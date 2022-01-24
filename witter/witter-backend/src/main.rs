#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))] // ^^ all this is to disable unused import warnings.

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate log;

use sqlx;
use std::env;
#[async_std::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    println!("{}", dotenv!("DATABASE_URL")); // Docs of dotenv: https://crates.io/crates/dotenv
    println!("Hello, world");

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello from api.") });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
// continue from:: https://youtu.be/yNe9Xr35n4Q?t=2113

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
