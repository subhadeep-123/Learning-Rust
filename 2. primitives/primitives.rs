// Rust have Scalar DataTypes and Primitive Data Types
fn main() {
    let logical: bool = true; // variable annotated

    println!("{}", logical);
    let a_float: f64 = 1.0; //Regular annotation
    let an_integer = 5i32; // Suffix annotation
    println!("float {}, integer {}", a_float, an_integer);

    // default value
    let default_float = 3.0;
    let default_integer = 7;

    // A type can also be inferred from context
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;
    println!("inferred_type: {}", inferred_type);

    // A mutable variable's value can be changed
    let mut mutable = 12;
    mutable = 21;

    // Error! The type of variable can't be changed
    let mutable = true;

    // Array signature consists of Type T and length as [T; length]
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Arr: {:?}", arr);

    // Tuple is collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("Tuple: {:?}", my_tuple);
}
