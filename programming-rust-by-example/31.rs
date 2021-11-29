fn main() {
    // ! this will throw error coz array can have only same type of elements and the type is obtained by the type of first element in the array.
    // ! error: mismatched types

    let arr1 = [1, "sahil"];
    // !  |         ^^^^^^^ expected integer, found `&str`
}
