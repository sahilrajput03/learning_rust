fn main() {
  // Offical types in rust: https://doc.rust-lang.org/std/index.html#primitives
  let x = 45;
  //Here, x is a i32 data type, i.e., a 32 bit signed integer.
  //Also,
  let y: i64 = 45;
  // that will make it a 64 bit signed integer.
  //-
  //When we know that the varialbe won't be a negative, we must use unsigned integer, like
  let z: u64 = 45;
  //coz unsigned integers don't support negative numbers.
  //-
  //We can use floating point data types by explicitly defining the data type, like
  let m: f64 = 6.7;
  let n: f32 = 7.2;
  //We can define boolean data type too, like
  let p: bool = false;
  //-
  //In all there are 10-15 data types in total, checkout in rust docs.
}
