// pg. 128
use std::fs::File;
use std::io::{self, Read};
fn read_from_file(file_name: &str) -> io::Result<String> {
    // Just like before, we open the file. Then we create a mutable String where the file content will be stored and finally we read all the file content at once with the read_to_string method (the String is reallocated as many times as needed). This method will fail if the string isn't proper UTF-8.
    //?  If everything went fine, we return our content
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    println!("hello");
    println!("Contents of file: {:?}", read_from_file("my_file.txt"))
    // Ok("amazing text..") // ? Successful file reading.
    // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" }) // ! Fail to read unexisting file, say fileName as my_file_101.txt
}
