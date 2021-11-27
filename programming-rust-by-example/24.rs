// pg. 66
// Irrefutable patterns, (it simply means destructuring in javascript).
// Another form of pattern matching is irrefutable patterns. A pattern is irrefutable when there's only one way to match it and it always succeeds. For instance, another way to get the elements of a tuple is with an irrefutable pattern:
fn main() {
    let tuple = (24, 42);
    let (a, b) = tuple;
    println!("{}, {}", a, b);
}
// In the second line, we assign the first element of the tuple to a and the second to b.
