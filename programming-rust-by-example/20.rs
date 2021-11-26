// pg. 62: While a structure allows us to get multiple values under the same variable, enumerations allow us to choose one value from different types of values.
// Enumerations
#[allow(dead_code)]
enum Expr {
    Add(i32, i32),
    Div { dividend: i32, divisor: i32 },

    Null,
    Sub(i32, i32),
    Mul(i32, i32),
    Val(i32),
}

#[allow(unused_variables)]
fn main() {
    println!("hello world");
    let quotient = Expr::Div {
        dividend: 10,
        divisor: 2,
    };
    let sum = Expr::Add(40, 2);
}
