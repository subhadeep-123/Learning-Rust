// === Generics in Rust ===

// Generics generalize types and functionalities, reducing code duplication.
// They require careful specification of valid types, leading to more complex syntax.

// Type Parameters:
// - Defined using angle brackets <T, U, ...>
// - Typically use uppercase letters like <T>
// - Represent generic types

// Generic vs. Concrete:
// - Generic: Uses type parameters (e.g., <T>)
// - Concrete: Specific types without generics

// Example: Generic function that accepts any type T
fn foo<T>(arg: T) {
    // T is a generic type parameter
    // Function can handle any type passed as arg
}

// Even if T is a previously defined struct, within foo<T>, T is treated as generic


// Concrete type `A`.
struct A;

struct Single(A);

struct SingleGen<T>(T);

#![allow(unused_variables)]
fn main() {
    let _s = Single(A);

    let _char:SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}