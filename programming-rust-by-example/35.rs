// page: 77
// ~sahil; return type as Option<(i32, i32) means ``None`` or a tuple with two values as i32 and i32. **also defined at pg.73.
fn min_max(slice: &[i32]) -> Option<(i32, i32)> {
    if slice.is_empty() {
        return None;
    }
    let mut min = slice[0];
    let mut max = slice[0];
    for &element in slice {
        if element < min {
            min = element;
        }
        if element > max {
            max = element;
        }
    }
    Some((min, max))
}
fn main() {
    // let my_tuple = (3, "sahil");
    // println!("{:?}", my_tuple);
    // ~sahil: tuple can have diffrent type of vlaues.

    println!("hello");
    let k = [5, 50, 500];
    println!("min and max: {:?}", min_max(&k));
    // Expected output: Some((5, 500))
}

//INTERESTED TEXT:: Here we return multiple values from a function by using a tuple. This time, & is on the left side of in, while previously it was on the right side of it; this is because this for loop is pattern matching against a reference by using &element.  This is something we can do in Rust, thus we don't need to dereference the element anymore with *.
