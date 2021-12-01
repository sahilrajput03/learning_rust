// pg. 126-127
use std::fs::File; // File is a type.
use std::io::{self, Write};
// we imported the io module (self) and the Write trait.
// we imported trait because we need use its methods

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    // ?  we call the static method create of the File type. If the file exists, it'll be truncated and if it doesn't, it'll be created. More information about this method can be found at https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create.

    // The `?` symbol. It's a syntactic sugar over the try! macro.  The try! macro is very simple to understand and its code can be resumed as this:
    // match result {
    //  Ok(value) => value,
    //  Err(error) => return Err(error),
    // }
    // ? Now let's look at this strange ? symbol. It's a syntactic sugar over the try! macro.  The try! macro is very simple to understand and its code can be resumed as this:
    // So that's pretty easy, but annoying to rewrite over and over, so the Rust teams decided to first introduce the try! macro and then after a long consensus, decided to add the ? syntactic sugar over it (it also works with the Option type). However, both code pieces are still working, so you can perfectly do as well:
    // let mut f = try!(File::create(file_name));
    // LEARN: RUST: ? is just syntatic sugar over try! in rust!

    // ? Alternatively, you can write the full version too:
    // let mut f = match File::create(file_name) {
    //     Ok(value) => value,
    //     Err(error) => return Err(error),
    // };
    f.write_all(content.as_bytes())
}
// It returns an io::Result type. It is an alias over the normal Result type (its documentation is at https://doc.rust-lang.org/stable/std/result/enum.Res ult.html) declared as follows:
// type Result<T> = Result<T, Error>;
// The only difference is that in case of error, the error type is already defined.
// So if our function worked without errors, it'll return Ok(()); it's the Ok variant containing an empty tuple which is considered the Rust equivalent of the void type. In case of error, it'll contain an io::Error, and it'll be up to you to handle it (or not). We'll come back to error handling a bit later.

fn main() {
    println!("hello");
    write_into_file("amazing text..", "my_file.txt");
}
