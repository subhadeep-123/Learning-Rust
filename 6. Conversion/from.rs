use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// It allows for a type to define how to create itself from another type
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My Number is {:?}", num);

    // Using Into
    let int = 5;
    let num: Number = int.into();
    println!("After Into: {:?}", num);
}
