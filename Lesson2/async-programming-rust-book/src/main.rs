#![allow(unused)]
mod m1;
mod m2;

fn m3() {
    let my_string = "Hello world!".to_string();
    println!("{my_string}", my_string = my_string);
}

fn main() {
    // m1::main();
    // m2::main();
    m3();
}
