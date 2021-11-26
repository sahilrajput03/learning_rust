// a structure allows us to get multiple values under the same variable:
fn main() {
    let point = Point { x: 24, y: 42 };

    println!("{}, {}", point.x, point.y);
    println!("{:?}", point) // :? Prints all properties in single line.
                            // Nevertheless, there's no standard way to display arbitrary structures
}

/* We can do
what the compiler suggests: using the {:?} syntax. That requires you to add an
attribute to the structure, so let's change it */
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

/* The #[derive(Debug)] attribute tells the compiler to automatically generate the code
to be able to print a debug representation of the structure. We'll see how this
works in the section about traits. */
