fn main() {
    let mut count: u32 = 0;

    println!("Let's count until infinity");

    // Infinite Loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");

            // exit from the loop
            break;
        }
    }
}
