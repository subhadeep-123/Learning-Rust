// There are 3 types of structs
//  1. Tuple structs, which are basically named tuples
//  2. The Classic C Structs
//  3. Unit struct, which are field-less, and useful for generics

// Attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("Point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    // if a field is already populated ..struct instance will populate other fields
    let bottom_right = Point {
        x: 10.5,
        ..another_point
    };
    println!("Bottom Right: {:?}", bottom_right);

    // `bottom_right.y` will be the same as `another_point.y` because we used that from `another_point`
    println!("Second Point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    println!("LeftEdge: {}, TopEdge: {}", left_edge, top_edge);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!("Rectangle: {:?}", _rectangle);

    //  Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a Tuple Struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("Pair contains {:?} and {:?}", integer, decimal);
}
