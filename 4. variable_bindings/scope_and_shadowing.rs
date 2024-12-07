// Variable bindings have a scop, and are constrained to live in a block.
// A block is a collection of statements enclosed by braces {}
fn main() {
    // Scope

    // The binding lives in the main function
    let long_lived_binding = 1;

    // The is a block, and has a smaller scope than the main function
    {
        // The binding only exists in this block
        let short_lived_binding = 5;

        println!("Inner short: {}", short_lived_binding);
    }
    println!("Outer long: {}", long_lived_binding);
    // End of the block

    // Variable Shadowing
    let shadowed_binding = 1;
    {
        println!("Before being shadowed: {}", shadowed_binding);

        // The binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("Shadowed in inner block: {}", shadowed_binding);
    }
    println!("Outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("Shadowed in outer block: {}", shadowed_binding);
}
