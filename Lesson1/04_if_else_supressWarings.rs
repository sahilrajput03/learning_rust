#[allow(unused_variables)]
#[allow(unused_parens)]
// #[allow(unused_variables, unused_parens)] //This is also legit too.

//src: https://stackoverflow.com/a/25877389
fn main() {
  let p = 3999;
  let m = 30;
  let n = 45;
  if n < 30 {
    println!("`n` is less than 30.");
  }

  if n == 30 {
    println!("`n` is equal to 30.");
  }

  if n > 30 {
    println!("`n` is greater than 30.");
  }

  if (m > 50) {
    println!("`m` is greater than 50.")
  } else {
    println!("`m` is lower than 50.")
  }

  //Similarly we can use, != , == , etc.
  //else is used in same manner as we do in js.
  //else if someCondition {
  //more code here.
}
//-
//The different thing is that using () i.e, parentheses is completely optional, and if you do use the #[warn(unused_parens)] is on by default.
//So, we can use parentheses but its not required.
