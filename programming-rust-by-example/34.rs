// ? Note: A partial equivalence relation is both symmetric and transitive, but not reflexive.
// ? The Eq trait is used when these three properties are satisfied.
// ~sahil, return type as Option<usize> means ``None`` or ``usize``
fn index<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (index, element) in slice.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}
// Here, we use again a generic type, but this time we use the PartialEq trait bound to be able to use the == operator on values of the T type. This function returns Option<usize>, meaning that it can either return no value (None) or the index (Some(index)). In the first line of the body, we use slice.iter().enumerate() to get the index in addition to the element of the slice. We use pattern matching right after the for keyword in order to assign the index and the element to variables. Inside the condition, we use the return keyword to return a value early. So if the value is found, it will return the index; otherwise, the loop will end and the None value is returned afterward.

fn main() {
    let array = [5, 6, 7, 8];
    println!("array: {:?}", index(&array, &5));
    println!("array: {:?}", index(&array, &8));
    println!("array: {:?}", index(&array, &50));
}
