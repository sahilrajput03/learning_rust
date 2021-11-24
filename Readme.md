# Readme

## Official Rust book

https://doc.rust-lang.org/book/

## Rust community

https://users.rust-lang.org/

https://t.me/rust_community

## Using rustmon

`rustmon fileName.rs` or `rmon fileName.rs`

For e.g., let say you have a rust program `myprogram.rs` then you'd need to run `rustmon myprogram` from the cli to run it in monitor mode.

## Formatter with vscode on code save

xxx note xxxx

### Simply use rust-analyzer and uninstall origianl rust vscode extension

Thats how tekipeps did, and it feels amazing as now i don't need to Cargo.toml in any opened folder in vscode so vsocode can format the files and `rustfmt.toml` works as expected!!
Yikes!

Simply install [this extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) and make sure that you have a cargo.toml file in the root of folder that you
have opened in vscode to make format on save works!

NOT WORKING: >>> Search for setting in vscode like: `rust rustfmt_path` and add path for your own global `rustfmt.toml` file. \*You must have official rust extension installed to
see this setting.

Global rustfmt.toml file location => PR: https://github.com/rust-lang/rustfmt/pull/3280 .

An ideal Cargo.toml file should be place in the root of folder opened in vscode i.e.,

(Bcoz Vscode's rust extension using command `cargo fmt --all` to format the code and throws error if the Cargo.toml file isn't formatted properly.)

```toml
[package]
name = "a-small-rust-app"
version = "0.1.0"

[[bin]]
name = "general"
path = "01_println.rs"
```

## Updating rustc via rustup

```bash
rustup update
```

## more..

### Tips:

0. Tutorial [here](https://www.youtube.com/playlist?list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL).

0.5. Was [here](https://www.youtube.com/watch?v=wM6nmsNcyic&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=39).

1. Whenever you see something like `car::bike::scooter` means that `car` is a module and `bike` is nested module inside `car` and `scooter` is a module nested inside `bike`.

2. strings: [amazing article](https://chercher.tech/rust/string-in-rust), or read from [amazing docs :LOL:](https://doc.rust-lang.org/rust-by-example/std/str.html).

3. [Gitignore](https://github.com/github/gitignore/blob/master/Rust.gitignore).

4. [Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html).

5. Reqwest

   - [Reqwest - some working examples @crates.io page, yikes!](https://crates.io/crates/reqwest),
   - [Reqwest docs](https://docs.rs/reqwest/0.11.0/reqwest/)

6. Rustler thing(popular idk why..): 1. [@docs](https://docs.rs/crate/rustler/0.21.1), 2. [@github](https://github.com/rusterlium/rustler),
   [@crates.io site](https://crates.io/crates/rustler).

### MISSION: → → → → → → finish till vid-42.

[\*\*Continue the vid-35 tutorial/and making this gist. @ here](https://youtu.be/B7koBE7VDGo)

### todo: Findout most popular web server from crate.io for rust??

[Follow a different course, and it feel good too](https://www.youtube.com/playlist?list=PLkO5ggdQuRaaeFke7nWS4ajhFVZ1biE7_)

## Rust important links:

- https://rust-lang-nursery.github.io/rust-cookbook/web/clients.html

- https://crates.io/ - The Rust community’s crate registry

- [https://www.rust-lang.org/](https://www.rust-lang.org/)

- [https://www.rust-lang.org/learn](https://www.rust-lang.org/learn) - Book, Course and Example.(Seems good) And other references to learn quickly.

- [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started) - A small rust app(demonstration, yikes).

- [Rust Plaground](https://play.rust-lang.org/)

- [Usage of semicolons in rust](https://stackoverflow.com/a/26665514/13994126 'Usage of semicolons in rust')

- [Official Conferences and playlists](https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA)

- Variable Shadoing: When we declare a binding in nested scope which is already declared in its parent scope. [@ wikipedia](https://en.wikipedia.org/wiki/Variable_shadowing)

- [Multipthreaded server @ rust docs](https://doc.rust-lang.org/book/ch20-02-multithreaded.html)

### Creating rust projects with cargo ([source](https://youtu.be/_RfxLg6K9oE))

```bash
$ cargo new hello-world-cargo --bin  # --bin tells the cargo to create a application project, not a library project.
$ cd hello-world-cargo
$ cargo run # This will output `Hello world!`. Congrats!!
$ `cargorun` or `cw` will run cargo in dev mode just like nodemon does, yikes!
```

The cargo application has entry point from `src/main.rs` file. **All rust programs begin with a `main` function.**

Firstly, anytime you make changes to `src/main.rs`, you need to run `cargo run` from cli. Secondly, `cargo.toml` file is similar to package.json file, as you'll find project info
and dependencies there. Thirdly, you'll get a git initiated project with pre-made `.gitignore` file.

### Compile and run with **rustc**

```rust
// File, main.rs
fn main(){
  println!("Hello world!")
}
```

Compile rust program via `rustc main.rs `and run it via `./main` (from bash) or `main.exe` (from command prompt).

### [cargo init](https://doc.rust-lang.org/cargo/commands/cargo-init.html)

```
--bin
Create a package with a binary target (src/main.rs). This is the default behavior.
--lib
Create a package with a library target (src/lib.rs).
```

## Snippets from the official rust vscode extension

src: https://github.com/rust-lang/vscode-rust#snippets

```txt
## Snippets
Snippets are code templates which expand into common boilerplate. IntelliSense includes snippet names as options when you type; select one by pressing enter. You can move to the next snippet 'hole' in the template by pressing tab. We provide the following snippets:

for - a for loop
macro_rules - declare a macro
if let - an if let statement for executing code only when a pattern matches
spawn - spawn a thread
extern crate - insert an extern crate statement
This extension is deliberately conservative about snippets and doesn't include too many. If you want more, check out Trusty Rusty Snippets.
```

So, fun.. below extension provides many rust snippets and officially recommended from rust vscode extension for snippets.

https://marketplace.visualstudio.com/items?itemName=polypus74.trusty-rusty-snippets
