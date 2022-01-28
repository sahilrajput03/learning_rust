#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))] // ^^ all this is to disable unused import warnings.

#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log;

use serde_json::json;
use sqlx::query;
use sqlx::PgPool;
use sqlx::Pool;
use std::env;
use tide::Request;
use tide::Server;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok(); // this is important else
    pretty_env_logger::init(); //? This prints all the beautiful logs.
                               // We ```env``` below to get value of environment variable instead of using macro like that >> But it works as well though. // println!("{}", dotenv!("DATABASE_URL")); // Docs of dotenv: https://crates.io/crates/dotenv

    let db_url = env::var("DATABASE_URL").unwrap();
    let db_pool: PgPool = Pool::new(&db_url).await.unwrap();

    println!("DATABASE_URL IS: {}", db_url);

    // let rows = query!("select 1 as one").fetch_one(&db_pool).await?;
    // dbg!(rows); // Prints our rows in the database.

    // let mut app: Server<()> = tide::new();
    let mut app: Server<State> = Server::with_state(State { db_pool });
    app.at("/").get(|req: Request<State>| async move {
        println!("request: get @ /");

        let json = json!([1, 2, 3]);

        Ok(json)
        // Ok("Hello from api.") // I am returning Ok() here.
    });

    app.at("/abc").get(|req: Request<State>| async move {
        println!("request: get @ /abc");
        Ok("Hello from api.") // I am returning Ok() here.
    });

    app.listen("127.0.0.1:8080").await.unwrap();
}
// continue from:: https://youtu.be/yNe9Xr35n4Q?t=2113

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    // ! Couldn't convert the error to `Error` <<<...LINE...>>> the trait `From<VarError>` is not implemented for `Error`
    // For line: ```let db_url = env::var("DATABASE_URL")?;``` compiler paniks and throws below error: //? So implement error VarError to supress it.
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
