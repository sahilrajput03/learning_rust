// pg. 61
fn main() {
    // Tuples can be used to return multiple values from a function. For instance, the str::split_at() method returns two strings:
    let (h, w) = "helloworld".split_at(5);
    //? this is like destructuring in js.
    println!("{}, {}!", h, w);
}
