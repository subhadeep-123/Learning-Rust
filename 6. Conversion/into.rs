use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

impl Number {
    fn show(&self) -> String {
        format!("The value is {}", self.value)
    }
}
fn main() {
    // it is simply the reciprocal of the From trait
    // Calling into() needs us to specify the result type as the compiler is unable to determine this most of the time

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
    println!("{}", num.show());
}
