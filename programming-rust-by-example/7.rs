#[allow(unused_variables)]
// pg: 54
// ! Error throwing program.
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;
    println!("{}", p1.x)
}

struct Point {
    x: i32,
    y: i32,
}

/*
When we do ```let p2 = p1``` , This means that we cannot use a value after it is moved.
In Rust, values are moved by default instead of being copied, except in some cases, as we'll see in
the next sub-section.
 */
