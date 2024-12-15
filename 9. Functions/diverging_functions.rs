// Diverging functions never return. They are marked using `!`, which is an empty type
fn foo() -> ! {
    panic!("This call never returns");
}

// As opposed to all the other types, this one cannot be instantiated, because the set of all possible
// values this type can have is empty. Note that, it is different from the () type,
// which has exactly one possible value.

// This function returns as usual, although there is no information in the return value.
fn some_fn() {
    ()
}

// #![feature(never_type)]
fn main() {
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");

    // This function, which will never return the control back to the caller
    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of this "addition" variable
            let addition: u32 = match i % 2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sun of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}
