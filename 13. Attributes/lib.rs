// The crate_type attribute can be used to tell the compiler whether a crate is a binary or a library
// (and even which type of library), and the crate_name attribute can be used to set the name of the crate.
// This crate is a library
#![crate_type = "lib"]
// This library is named "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("Called rary's `public_function()`");
}

fn private_function() {
    println!("Called rary's `private_function()` ");
}

pub fn indirect_access() {
    print!("Called rary's `indirect_access()`, that\n> ");
    private_function();
}
