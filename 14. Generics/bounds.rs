use std::fmt::{Debug, Display};

// The type parameters often must use traits as bounds to stipulate what functionality a type implements
// For example, the following example uses the trait Display to print and so it requires T to be bound by Display;
// that is, T must implement Display

// Define a function `printer` that takes a generic type `T` which must implement trait `Display`
fn printer<T: Display>(t: T) {
    println!("{}", t)
}

// Bounding restricts the generic to types that conform to the bounds
struct S<T: Display>(T);

// Another effect of bounding is that generic instances are allowed to access the methods of traits specified in the bounds
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    // This will not work, because `Vec<T>` does not implement `Display`
    // let s = S(vec![1]);
    // println!("{}",s);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
    println!("Area: {}", rectangle.area());

    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
    // | Error: Does not implement either `Debug` or `HasArea`.
}
