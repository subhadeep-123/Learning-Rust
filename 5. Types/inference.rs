// The type inference engine considers both initialization and usage.

fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // The compiler does not know the exact type if `vec`, only it's a vector of something `Vec<_>`

    // Insert `elem` in the vector
    // vec.push(elem);
    // Now the compiler knows that `vec` is a vector of `u8`

    println!("{:?}", vec);
}
