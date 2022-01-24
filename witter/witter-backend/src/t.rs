fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    println!("{}", dotenv!("DATABASE_URL")); // Docs of dotenv: https://crates.io/crates/dotenv
    println!("Hello, world");

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello from api.") });
}
