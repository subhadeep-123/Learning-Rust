mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }

        pub fn get_contents(&self) -> &T {
            &self.contents
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox {contents: "Public Information"};

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // We cannot do this because `contents` is a private field
    // let closed_box = my::ClosedBox { contents: "classified information" };

    // However, structs with private fields can be created using public constructors
    let closed_box = my::ClosedBox::new("Classified information");

    println!("Contents of closed box: {}", closed_box.get_contents());
}