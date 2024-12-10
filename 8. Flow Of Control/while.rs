// while keyword can be used to run a loop while a condition is true
fn main() {
    // Counter variable
    let mut n = 1;

    // loop while `n` is less than 101
    while n < 50 {
        if n % 15 == 0 {
            println!("frizzbuzz");
        } else if n % 3 == 0 {
            println!("frizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment Counter
        n += 1;
    }
}
