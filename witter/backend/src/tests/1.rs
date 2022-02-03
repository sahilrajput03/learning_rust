// use dotenv;

// use serde_json::json;
// use sqlx::http::StatusCode;
// use sqlx::query;
// use sqlx::PgPool;
// use sqlx::Pool;
// use tide::http::StatusCode;
// use tide::Server;

// #[async_std::main]
// async fn main() {
//     let app = server().await;

//     app.listen("127.0.0.1:8080").await.unwrap();
// }

// async fn server() -> Server<State> {
//     dotenv::dotenv().ok();
//     pretty_env_logger::init();

//     let db_url = std::env::var("DATABASE_URL").unwrap();
//     let db_pool: PgPool = Pool::new(&db_url).await.unwrap();

//     let mut app: Server<State> = Server::with_state(State { db_pool });

//     //app.at("/).get
// }
