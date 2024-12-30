// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    // `c` is destroyed and the memory freed
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    // x is of type u32, which is a primitive type.
    // Primitive types (like integers, floats, and char) are stored on the stack.
    // When you do let y = x;, x is copied to y. This is because primitive types implement the Copy trait.
    // Since the stack is small and copying is cheap, Rust duplicates the value without moving ownership.
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    // a is a Box<i32>, which is a heap-allocated value. The Box itself (the pointer) is stored on the stack, but it points to data (5i32) on the heap.
    // When you do let b = a;, the ownership of the heap-allocated data is transferred (moved) to b.
    // a no longer has access to the data because Rust prevents multiple owners of heap-allocated memory (to avoid double frees and dangling pointers).
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);
}
