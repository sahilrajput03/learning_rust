//  leaving page 67, i.e., topic of 'traits' and this seems really interesting though.
// Learn: Bitwise operator give value like:
//? src: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_NOT#description
//? ~x = -(x+1) (<< in compiler of js).

// rust: Bitwise not operator is ! in Rust
//? List of all operators in rust: https://doc.rust-lang.org/book/appendix-02-operators.html#operators
// rust; ! act as both Bitwise or logical complement:

trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
    // we could have defined the function body of any of the above method, check pg. 69.
}
impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index);
        //? Learn rust: Bitwise AND and assignment: var &= expr
        //? Learn rust: Left-shift operator: <<
    }
    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
        //? Learn rust: Right-shift operator: >>
    }
    fn set(&mut self, index: usize) {
        *self |= 1 << index;
    }
}
fn main() {
    let mut num = 2;
    println!("I got:1: {}", num);
    num.set(15);
    println!("I got:2: {}", num.is_set(15));
    println!("I got:2.1: {}", num.is_set(16));
    println!("I got:3: {}", num);
    num.clear(15);
    println!("I got:4: {}", num);
}
