fn main() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m)
        }
    }
}
