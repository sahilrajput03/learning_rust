# README

Making project following from: https://www.youtube.com/watch?v=yNe9Xr35n4Q

~ David Pedersen

**creating postgres database**

```bash
createdb --help
# createdb creates a PostgreSQL database.

# Usage:
createdb witter # creates a database named witter in postgres.
```

## CRATES I am using:

Source: https://crats.io
Source: https://youtu.be/yNe9Xr35n4Q

- https://crates.io/crates/actix
- https://crates.io/crates/warp (its a web framework and it uses tokio internally for async functionality, it has web sockets support)
- pretty_env_logger: https://crates.io/crates/pretty_env_logger
- [✓] https://crates.io/crates/tide (we're are using tides in our web-server project. Yo!!)
- https://crates.io/crates/rocket (rocket is a web framework but its downside is like it doens't support async code).

- [✓] sqlx: https://crates.io/crates/sqlx (sqlx is better than diesel coz it has better error handling and it's easier to use.)
- diesel: https://crates.io/crates/diesel

- [✓] async-std: https://crates.io/crates/async-std

- [✓] yew: https://crates.io/crates/yew (its frontend framework and it'll build web assembly code so our rust runs on browser)
- maud: https://crates.io/crates/maud ( 1.1k stars on github; it is provides a macro html! which is analogous to jsx in javasrcipt)
- [] typed-html: https://crates.io/crates/typed-html ( ITS BETTER SUPPORTED THAN `maud`; 1.7k stars on github; it is a macro for html! which is analogous to jsx in javasrcipt)

- [✓] dotenv_codegen: https://crates.io/crates/dotenv
