// If we declare a function takes takes a closure as parameter, then any function that
// satisfies the trait bound of that closure can be passed as a parameter.
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure");

    call_me(function);
    call_me(closure);
}
