// !! MASTER's Palace.
// ? Self` is only available in impls, traits, and type definitions ~SAHIL ~from compiler.
// self when used as first method argument, is a shorthand for self: Self. There are also &self, which is equivalent to self: &Self, and &mut self, which is equivalent to self: &mut Self.
// Self in method arguments is syntactic sugar for the receiving type of the method (i.e. the type whose impl this method is in). This also allows for generic types without too much repetition. src: https://stackoverflow.com/a/32306132/10012446
// #[allow(dead_code)]
fn change_value(a: &mut i8) {
    println!("I got: {}", a);
    *a = 20;
    // ! Below statement would throw error.
    // a = 20
    //?    ^^ expected `&mut i8`, found integer
    //? consider dereferencing here to assign to the mutable borrowed piece of memory
}
fn p(a: &i8) {
    println!("value is:{}", a)
}
fn p1(a: i8) {
    println!("value is:{}", a)
}
fn main() {
    let mut a = 127; // ! TIP1:
    p1(a); // pass value
    p(&a); // pass reference.
    change_value(&mut a); // pass mutable reference.
                          // You should only pass a mutable reference if you really want to mutatae the bindings's value.
                          // learn rust: In below examples all values print correctly with no error coz println macro deferences all the values automatically even if you pass any reference type to it.
    println!("{}", &a);
    println!("{}", &&a);
    println!("{}", *&a);
    println!("{}", *&a);
    println!("{}", a)
}

//TIP1: set value as 128 and that'll throw nice error of value overflow form function change_value coz u have defined input type as i8 which has max value possible till 127 only.
