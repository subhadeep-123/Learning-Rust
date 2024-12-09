use std::mem::size_of_val;

// Add a type suffix to numeric literals, like 42i32.
// Without a suffix, integers default to i32 and floats to f64 based on context.

fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("Size of `x` in bytes: {}", size_of_val(&x));
    println!("Size of `y` in bytes: {}", size_of_val(&y));
    println!("Size of `z` in bytes: {}", size_of_val(&z));
    println!("Size of `i` in bytes: {}", size_of_val(&i));
    println!("Size of `f` in bytes: {}", size_of_val(&f));
}
