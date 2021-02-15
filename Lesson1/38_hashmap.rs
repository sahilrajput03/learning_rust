use std::collections::HashMap;
// hashmaps is just a collection of key-value pairs.

fn main() {
  let mut marks = HashMap::new();
  // Add values
  marks.insert("Rust programming", 96);
  // marks.insert(key, value)
  marks.insert("Web Development", 94);
  marks.insert("UX Design", 75);
  marks.insert("Professional Computing Studies", 45);

  // Find length of HashMap
  println!("How many subjects have you studies? {}", marks.len());
  //Output: How many subjects have you studies? 4

  // Get value for a key
  match marks.get("Web Development") {
    Some(mark) => println!("You got {} for Web Dev!", mark),
    None => println!("You didn't study web development."),
    //Optionally you can use  _ => println!("You didn't study web development."),
  }

  //Remove a value
  marks.remove("UX Design");

  println!("\n");
  // Loop through HashMap
  for (subject, mark) in &marks {
    println!("For {} you got {}%!", subject, mark);
  }

  // Check for value
  println!(
    "Did you study C++ {}",
    marks.contains_key("C++ Programming")
  );
}
