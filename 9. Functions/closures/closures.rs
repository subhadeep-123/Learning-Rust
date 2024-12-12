// Closures are functions that can capture the enclosing environments.

fn main() {
    let outer_var = 42;

    // As the compiler says "can't capture dynamic environment in a fn item"
    // fn function(i: i32) -> i32 {
    //     i + outer_var
    // }

    // Closures are anonymous, here we are binding them to references.
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closures_annotated = |i: i32| -> i32 { i + outer_var };
    let closures_inferred = |i| i + outer_var;

    // Call the closures
    println!("Closures Annotated: {}", closures_annotated(1));
    println!("Closures Inferred: {}", closures_inferred(1));
    // Once a closures type is inferred, it can't be changed to anything else

    // A closure taking no arguments which returns an `i32`
    let one = || 1;
    println!("Closure returning one: {:?}", one());
}
