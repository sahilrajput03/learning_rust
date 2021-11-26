// pg 57 - Copy types
// We can make our own types Copy by adding derive to them:

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
//? Since Copy requires Clone, we also implement the latter for our Point type. We cannot derive Copy for a type containing a value that does not implement Copy.
//? Now, we can use a Point without having to bother with references:

fn print_point(point: Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

#[allow(unused_variables)]
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;
    print_point(p1);
    println!("{}", p1.x);
    //                  ^^^ this works good.
}
