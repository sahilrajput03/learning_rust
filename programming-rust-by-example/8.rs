// pg: 54
// ! Error throwing program.
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = &p1; // p2 is a reference to p1, i.e. &p1 instead of copying i.e., p2 = p1
                  // This code compiles and, in this case, p2 is a reference to p1, which means that it points to the same memory location. Rust ensures that it is always safe to use a reference, since references are NOT pointers, they cannot be NULL.(i.e., Pointers can be NULL ~Sahil)
    println!("{}", p2.x)
}

#[allow(dead_code)]
//      ^^ this is needed to prevent the warning i.e., ````warning: field is never read: `y` ````
struct Point {
    x: i32,
    y: i32,
}
