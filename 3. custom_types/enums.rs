// The enum keyword allows the creation of a type which may be one of a few different variant.
// Any variant which is valid as a struct is also valid in an enum

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    PageLoad,
    PageUnload,
    // like tuple struct
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and returns Nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure 'c' from inside the 'enum' variant
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),

        // Destructure `Click` into `x` and `y`
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place you'll see this is in impl blocks using the self alias
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned` created an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // We can refer to each variant via its alias, not its long and inconvenient name
    let x = Operations::Add;
    println!("Sum of 10 + 20 {}", x.run(10 as i32, 20 as i32));
}
