fn main() {
    println!("Number literal with type: {:?}", 23u8);
    //                                         ^^^^ number with type unsigned 8 bit integer.
    println!("Number literal with type: {:?}", 23i64);
    //                                         ^^^^^ number with type signed 64 bit integer.
    println!("----");
    let arr1 = [1, 2, 3, 4];

    let arr2: [i16; 4] = [1, 2, 3, 4];
    //^^ Specifies the type and size of an array.

    let arr3 = [1u8, 2, 3, 4];
    //^^ An alternative way to define type for array is to use a literal suffix:

    let arr4 = [0u8; 5]; // Declares 5 elements in the array as 0// Declares 5 elements in the array as 0.

    println!("we have arr1:{:?}", arr1);
    println!("we have arr2:{:?}", arr2);
    println!("we have arr3:{:?}", arr3);
    println!("we have arr4:{:?}", arr4);
}
// A literal suffix is the composition of a literal (that is, a constant) and a type suffix, so with the 1 constant and the u8 type, we get 1u8. Literal suffixes can only be used on numbers. This declares an array of 4 elements of the u8 type.
