#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}c is above 30 Celsius", t),
        // If condition part is a guard
        Temperature::Celsius(t) => println!("{}c is equal to or below 30 celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    // Compiler won't take guard condition into account when checking if all patterns are covered
    // by match expression

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
    }
}
