#[allow(unused_mut)]
// HMM>>> learn some more via this program!!
fn main() {
  let mut x = 10;

  let dom = &mut x;
  *dom += 1;

  println!("x is {}", x);
  // println!("dom is {}", dom);
  // ↑↑↑↑ Only printing this line throws error.
  // This throws error and says to read @
  // `rustc --explain E0502` command.
}

// fn name(arg: Type) -> RetType {
//   unimplemented!();
// }
