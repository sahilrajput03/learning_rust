# README

Making project following from: https://www.youtube.com/watch?v=yNe9Xr35n4Q

~ David Pedersen

**return ?**

```rs
function ...{
	Ok(json)
	// return Ok(json); // THIS IS THE SAME AS ABOVE.
}
```

**Inferring**:

```rs
app.at("/").get(|req: Request<_>| async move {
// Here Request<_> the compiler will infer the type of the request in place where we have _ in there.

// We can do this in other places as well like:
let mut my_vector: Vec<_> = vec![10, 20, 30, 40]; // while defining a vector.
// Refer example 30.rs in Lesson1 for implementation.
```

**Something about async functions:**

This is an awesome way we can detect the types required from the return of any library function. We can apply this rule anywhere in the code to get that.

````rs
	/*
	How do we know that async function has to return something that implements a Future ??
	let fa: () = async { 1 };

	Above throw error like:
	let fa: () = async { 1 };
			^   ^^^^^^^^^^^ expected `()`, found opaque type
			^
			^expected due to this

	note: expected unit type `()`
	found opaque type `impl Future`
			^^^^^^^^^^^^^^^^^^^^^^^^^ ~SAHIL: This tells that that fa's type has to be something that implements Future for sure.

	*/

``

**Unit type**:

> For convenience and historical reasons, the tuple type with no fields (()) is often called unit or the unit type. Its one value is also called unit or the unit value.

Source: https://doc.rust-lang.org/reference/types/tuple.html#tuple-types

**creating postgres database**

```bash
createdb --help
# createdb creates a PostgreSQL database.

# Usage:
createdb witter # creates a database named witter in postgres.
````

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
