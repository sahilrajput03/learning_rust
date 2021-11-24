// pg: 54
// ! Error throwing program.
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = &p1; // To avoid moving a value, we can take a reference to it by prefixing it with &:
                  // This code compiles and, in this case, p2 is a reference to p1, which means that it
                  // points to the same memory location. Rust ensures that it is always safe to use a
                  // reference, since references are not pointers, they cannot be NULL.(i.e., Pointers can be NULL ~Sahil)
    println!("{}", p2.x)
}

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}
