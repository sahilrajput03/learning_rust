// Arrays are fixed-size, but if we want to create a function that works with arrays of any size, we need to use another type: a slice.
fn first<T>(slice: &[T]) -> &T {
    &slice[0] // we return reference of first element.
}
fn main() {
    let k = [5, 6, 7, 8, 9];
    println!("First element: {}", first(&k));
    println!("Passing complete array: {}", first(&k[..]));

    println!("Passing array from 2nd index: {}", first(&k[2..]));
    //  &k[2..] means sliced array of k from 2nd index to end of the array.

    println!("Passing only first 4 items: {}", first(&k[..4]));

    println!(
        "Passing 3rd index to 5 index both inclusive: {}",
        first(&k[3..5])
    );
}
