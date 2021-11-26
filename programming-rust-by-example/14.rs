// pg. 58 - Mutable References
// #[derive(Clone, Copy)]
#[allow(unused_variables, dead_code)]
struct Point {
    x: i32,
    y: i32,
}

fn inc_x(point: &mut Point) {
    // ?        ^^ We say point is a reference to a mutable Point type.
    // point.x += 1 // works good!
    point.x += 1
}

#[allow(unused_variables, dead_code)]
fn main() {
    let mut p1 = Point { x: 11, y: 2 };
    inc_x(&mut p1); // Remember earlier it was getting moved into the function we don't pass it like: ```print_point(p1)```.
    println!("{}", p1.x);
    // ? inc_x(&p1); // this would throw error coz of mismatch type of argument we passed vs. paramater type we assinged in funciton.
}
