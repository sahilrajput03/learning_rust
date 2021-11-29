// pg. 78
// for reference for contextual use of BitSet refer 25.rs file ~sahil.
trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
    // we could have defined the function body of any of the above method, check pg. 69.
}

macro_rules! int_bitset {
    ($ty:ty) => {
        impl BitSet for $ty {
            fn clear(&mut self, index: usize) {
                //  The isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
                *self &= !(1 << index);
            }
            fn is_set(&self, index: usize) -> bool {
                (*self >> index) & 1 == 1
            }
            fn set(&mut self, index: usize) {
                *self |= 1 << index;
            }
        }
    };
}
// ? A macro can have multiple rules, similar to match arms, but it matches on Rust syntactic elements instead, with types, expressions, blocks of code, and so on.  Here we only have one rule and it matches against a single type since we use :ty. The part before :ty ($ty) is the name for the element that was matched. Inside the curly brackets, after the => symbol, we see the actual code that will be generated. It is the same as our previous implementation of BitSet for u64, except that it uses the ```metavariable``` $ty instead of u64.

//
int_bitset!(i32);
int_bitset!(u8); // ? if u comment this line then u'll get error like .set method not found in u8.
int_bitset!(u64); //? if u comment this line then u'll get error like .set method not found in u8.

fn main() {
    println!("hello");
    let mut a: i32 = 1;
    let mut b: u8 = 1;
    let mut c: u64 = 1;
    a.set(2);
    b.set(2);
    c.set(2);
    println!("a is 2: {}", a.is_set(2));
    println!("a is 2: {}", a.is_set(2));
    println!("a is 2: {}", a.is_set(2));
    // LEARN: ~sahil
    // ! k = int_bitset!(i32);//COMPILER ERROR: in this macro invocation | = note: the macro call doesn't expand to an expression, but it can expand to a statement
}
