#![allow(dead_code, unused)]
// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on; // block_no blocks the current thread.

async fn hello_world() {
    println!("hello, world!");
}

#[allow(dead_code)]
fn test1() {
    let future = hello_world(); // Nothing is printed
    println!("Future has been created.."); // Prints text.
    block_on(future); // `future` is run and "hello, world!" is printed
    println!("Statement after future execution.")
}

async fn test2() {
    //? LEARN: We can only us .await in a async function only.
    hello_world().await; //? LEARN: Using ```.await```  waits for the completion of another type that implements the Future trait, such as the output of another async fn.
    println!("#2:Future has been created..");
    println!("#2:Statement after future execution.")
}

pub fn main() {
    println!("EXECUTING module m1::");
    // test1();
    block_on(test2());
}
