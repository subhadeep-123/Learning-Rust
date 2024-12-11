fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");

    match age {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 12 ..=19 => println!("I'm a teen of age {:?}", n)
        n if n > 0=> println!("I'm a old person of age {:?}", n)
    }

    match some_number() {
        // Got `Some` variant, match if its value bound to `n`
        // is equal to 42
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n)
        _ => ()
    }
}