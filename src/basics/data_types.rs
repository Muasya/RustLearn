// use std::env::var;


//datatypes in rust

// ***** strings *****
// string is a sequence of characters
// string is a sequence of bytes

// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");
  
    // Get length
    println!("Length: {}", hello.len());
  
    // Push char
    hello.push('W');
  
    // Push string
    hello.push_str("orld!");
  
    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
  
    // Check if empty
    println!("Is Empty: {}", hello.is_empty());
  
    // Contains
    println!("Contains 'World' {}", hello.contains("World"));
  
    // Replace
    println!("Replace: {}", hello.replace("World", "There"));
  
    // Loop through string by whitespace
    for word in hello.split_whitespace() {
      println!("{}", word);
    }
  
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
  
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
  
    println!("{}", s);
  }


/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run_2() {
  // Default is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 4545445454545;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Get boolean from expression
  let is_greater: bool = 10 < 5;

  let a1 = 'a';
  let face = '\u{1F600}';

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}




