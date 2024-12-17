// #[outer_attribute] applies to the item immediately following it.
// Examples of items are: a function, a module declaration, a constant, a structure, an enum
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// #![inner_attribute] applies to the enclosing item (typically a module or crate).
// This attribute is interpreted as applying to the entire scope in which it's placed
#![allow(unused_variables)]
fn main() {
    let x = used_function();
}

// Attributes can take arguments with different syntaxes:
// #[attribute = "value"]
// #[attribute(key = "value")]
// #[attribute(value)]

fn used_function() -> i32 {10}

// `#[allow(dead_code)]` os an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_functions() {}