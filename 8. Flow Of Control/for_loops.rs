// The `for in` construct cab be used to iterate through an `Iterator`
// FrizzBuzz using for instead of while

fn for_and_range() {
    for n in 1..=51 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // `iter` borrows each element of the collection through each iteration
    for name in names.iter() {
        match *name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn for_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // `into_iter` consumes the collection so that on each iteration the exact data is provided
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn for_iter_mut() {
    // THis mutably borrows each element of the collection, allowing for the collection to be modified in place

    let mut names = vec!["Subhadeep", "Richard", "Argha"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Subhadeep" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
fn main() {
    for_and_range();

    for_iter();
    for_into_iter();
    for_iter_mut();
}
