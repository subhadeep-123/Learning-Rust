// Break and continue from outer loops when dealing with nested loops.
// In this cases the loops must be annotated with some 'label
// The label must be passed  to the break/continue statement

#![allow(unreachable_code, unused_labels)]
fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
