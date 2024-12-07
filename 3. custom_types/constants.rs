// Constants
// const: An unchangeable value (the common case)
// static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and
//         does not have to be specified. Accessing or modifying a mutable static variable is unsafe

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant is some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("The is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" })
}
