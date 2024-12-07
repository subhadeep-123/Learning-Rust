// It's possible to declare variable binding first, and initialize them later.
fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding;
        a_binding = x * x;
    }

    println!("A binding: {}", a_binding);
}
