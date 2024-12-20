// If a trait doesn't include any functionality we can still use it as a bound
// Eq and Copy are examples of such traits from the std library

struct Cardinal;
struct BlueJay;
struct Turkey;

// Traits with no methods are called Marker traits
trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these traits.
// The fact that the traits are empty is irrelevant
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` will not work on a blue jay nor vice versa
    // because of the bounds
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}
