use std::fmt;

// This structure cannot be printing
struct UnPrintable(u32);

impl fmt::Display for UnPrintable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My Tuple Struct Value: {}", self.0)
    }
}
// derive attribute automatically creates the implementation
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
    let dpt = DebugPrintable(42);
    println!("Print DebugPrintable Struct\n{:?}", dpt);

    let up = UnPrintable(20);
    println!("{}", up);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);

    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    )
}
