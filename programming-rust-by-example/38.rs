// pg. 81, amazing explanation @ pg. 82
// A macro helps to avoid code repetition.
// A macro in rust is a code generator, i.e., whatever you pass in the macro's calling it can be used to generate awesomely amazing wide variety of code which would be tedious to write each time. Basically it provides a way of making abstractions(which aren't possible by functions obviously). ~ sahil.
//?~sahil~COMPILER~Say I put ``exprr`` in place in expr then below error is thrown:
//! help: valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`
macro_rules! hash {
    ($( $key:expr => $value:expr ),*) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
    }};
}

fn main() {
    let my_hash = hash! {
        "one" => 1,
        "two" => 2
    };
    println!("I made hashmap:{:?}", my_hash);
}
//
// let mut my_hash = {
//  ::std::collections::HashMap::new();
//  hashmap.insert("one", 1);
//  hashmap.insert("two", 2);
//  hashmap
// }
