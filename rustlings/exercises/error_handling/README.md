# Error handling

~sahil
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html

https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html

https://doc.rust-lang.org/std/num/struct.ParseIntError.html


https://doc.rust-lang.org/std/collections/struct.HashMap.html

https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-va

https://doc.rust-lang.org/std/vec/struct.Vec.html

https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

https://doc.rust-lang.org/book/ch05-01-defining-structs.html


Most errors aren’t serious enough to require the program to stop entirely. 
Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)
