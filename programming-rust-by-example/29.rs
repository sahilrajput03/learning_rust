// pg. 73
// Generics can also be used in a type. The Option type from the standard library is a generic type, defined as such:
enum Option<T> {
    Some(T),
    None,
}
// This type is useful to encode the possibility of the absence of a value. None means no value, while Some(value) is used when there's a value.
