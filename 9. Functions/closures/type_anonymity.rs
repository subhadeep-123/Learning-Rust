// When you define a closure in Rust, the compiler automatically creates an anonymous (unknown)
// type for it. This type stores any variables that are captured from its enclosing environment.

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;

    // Captures `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);

    apply(print);
}
