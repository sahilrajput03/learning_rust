fn main() {
    // ~sahil --> array is not moved even thought the book says it would be moved if used directly without a reference operator @ line #1. PROOF:: I used array again on line #2.
    let array = [1, 2, 3, 4]; // LINE:#1
    let mut sum = 0;
    for element in array {
        sum += element;
    }
    println!("Sum: {}", sum);
    println!("array: {:?}", array); // LINE:#2
}
