// Functions that take one of more functions and/or produce a more useful function.
// HOFs and lazy iterators give Rust its functions flavor

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all numbers with odd squares under 1000");

    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2,.. to infinity
    for n in 0.. {
        let n_sq = n * n;

        if n_sq >= upper {
            break;
        } else if is_odd(n_sq) {
            // Accumulate value, if it's odd
            acc += n_sq;
        }
    }

    println!("Imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
