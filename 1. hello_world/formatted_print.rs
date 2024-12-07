use std::fmt;

fn main() {
    // Automatic Replacement with Arguments
    println!("{} days", 31);

    // Positional Arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named Arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character aster a `:`
    println!("Base 10:                  {}", 69420);
    println!("Base 2 (binary):          {:b}", 69420);
    println!("Base 8 (octal):           {:o}", 69420);
    println!("Base 16 (hexadecimal):    {:x}", 69420);

    // right-justify text with specified width.
    println!("{number:>5}", number = 1);

    // Pad numbers with extra zeros
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used
    println!("My name is {0}, {1}, {0}", "James", "Bond");

    #[allow(dead_code)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Structure with value {}", self.0)
        }
    }
    println!("This struct `{}` won't print...", Structure(3));

    // Capture arguments from surrounding variables
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // Print pi
    let pi = std::f64::consts::PI;
    println!("The value of Pi is approximately: {:.5}", pi);
}
