/*
Sometimes, the structure contains a lot of nested fields and this representation is
hard to read. To remedy that, we can use the {:#?} syntax to pretty-print the
value:
 */

fn main() {
    let point = Point { x: 24, y: 42 };

    println!("{}, {}", point.x, point.y);
    println!("{:#?}", point) // :#? Prints each property in new line.
                             // Nevertheless, there's no standard way to display arbitrary structures
}

#[derive(Debug)] // <== Get this fast via typing #[der<Tab>
struct Point {
    x: i32,
    y: i32,
}

// Other formatting syntax: https://doc.rust-lang.org/stable/std/fmt/
