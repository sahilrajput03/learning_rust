#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))] // ^^ all this is to disable unused import warnings.

#nmacro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log;

use serde_json::json;
use sqlx::{query, PgPool, Pool};
// use sqlx::http::StatusCode;
use std::env;
use tide::{Request, Response, Server, http::StatusCode};

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
        // println!("request: get @ /");
        let json = json!([1, 2, 3]);
        Ok(json)
        // Ok("Hello from api.") // I am returning Ok() here.
    });

    app.at("/abc").get(|req: Request<State>| async move {
        println!("request: get @ /abc");
        Ok("Hello from api.") // I am returning Ok() here.
    });

    // app.listen("127.0.0.1:8080").await.unwrap();
	app
}
// continue from:: https://youtu.be/yNe9Xr35n4Q?t=6175

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}

#[cfg(test)]
mod test {
	#[allow(unused_imports)]
	use super::*;
	
	use futures:{executor::block_on, prelude::*};
	use http_service::{HttpService, Response};
	use http_types::{Method, Request, Url};

	pub struct TestBackend<T: HttpService> {
		service: T,
		connection: T::Connection,
	}

	impl<T: HttpService> TestBackend<T> {
		fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
			Ok(Self {
				service,
				connection,
			})
		}

		pub fn simulate(
			&mut self,
			req: Request,
		) -> Result<Response, <T::ResponseFuture as TryFuture>::Error>{
			block_on(
				self.service
				.respond(self.connection.clone(), req)
				.into_future(),
			)
		}
	}

	pub fn make_server<T: HttpService>(
		service: T,
	) -> Result<TestBackend<T>, <T::ConnectionFuture as TryFuture>::Error> {
		TestBackend::wrap(service)
	}
	
	
	#[async_std::test]
	async fn nested() -> tide::Result<()> {
		let server = server().await;
		let mut server = make_server(server).unwrap();

		let req = Request::new(
			Method::Get,
			Url::parse("Http://example.com/").unwrap(),
		);
		let req = Request::new(
			Method::Get,
			Url::parse("http://example.com/").unwrap();
		)
		let res = server.simulate(req).unwrap();
		assert_eq!(res.status(), 200);
		assert_eq!(res.body_string().await.unwrap(), "[1,2,3]"]);

		
		// let mut inner = tide::new();
		// inner.at("/foo").get(|_| async { Ok("foo") });
		// inner.at("/bar").get(|_| async { Ok("bar") });

		// let mut outer = tide::new();
		// Nest the inner app on /foo
		// outer.at("/foo").nest(inner);

		// assert_eq!(outer.get("/foo/foo").recv_string().await?, "foo");
		// assert_eq!(outer.get("/foo/bar").recv_string().await?, "bar");
		Ok(())
	}

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

