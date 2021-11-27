use std::ops::Add;
// Importing add trait from standard library, we'll use it to implement in out type.

#[derive(Debug)]
struct Point {
    x: i8,
    y: i8,
}
impl Add<Point> for Point {
    type Output = Point; // ~sahil, directly putting Point in place of Output in below line wont' work as it'll throw error like: ```associated type not found```

    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("p3: {:?}", p3)
}
// Having to specify the output parameter with an associated type (instead of setting it to Self) gives us more flexibility. For instance, we could implement the scalar product for the * operator, which takes two Points and returns a number.
// You can find all the operators that can be overloaded on this page, at https://doc.rustlang.org/stable/std/ops/index.html.
// Since Rust 1.20, Rust also supports associated constants in addition to associated types.
