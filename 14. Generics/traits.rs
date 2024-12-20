// Traits can also be generic

// Non-copyable types
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and caller `U`
// This is a blanket implementation that says: for any types T and U, type U implements DoubleDrop<T>.
// This means every type in Rust automatically has an implementation of DoubleDrop for any type T.
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed argument, deallocation both.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);
}
