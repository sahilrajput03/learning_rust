// pg 57
// copy types: Some types are not moved when we assigned a value of these types to another variable. This is the case for basic types such as integers. For instance, the following code is perfectly valid:
#[allow(unused_variables)]
fn main() {
    let num1 = 42;
    let num2 = num1;
    println!("{}", num1);
}

// We can still use num1 even thought we assigned it to num2. This is because the basic types implement a special marker: Copy. Copy types are copied instead of moved.
