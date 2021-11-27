fn main() {
    let arr1 = [1, 2, 3, 4];
    let arr2: [i16; 4] = [1, 2, 3, 4];
    //^^ Shows how to specify the type of an array.
    // An alternative way to define type for array is to use a literal suffix:
    let arr3 = [1u8, 2, 3, 4];

    println!("we have arr1:{:?}", arr1);
    println!("we have arr2:{:?}", arr2);
    println!("we have arr3:{:?}", arr3);
}
// A literal suffix is the composition of a literal (that is, a constant) and a type suffix, so with the 1 constant and the u8 type, we get 1u8. Literal suffixes can only be used on numbers. This declares an array of 4 elements of the u8 type.
