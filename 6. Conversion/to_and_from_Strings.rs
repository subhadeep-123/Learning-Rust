use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// To obtain functionality on a user defined type simple implement the FromStr
impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(radius) => Ok(Circle { radius }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    // Converting to String
    let circle = Circle { radius: 6 };
    println!("{circle}");
    println!("{}", circle.to_string());

    // Parsing a String
    let parsed: i32 = "5".parse().expect("Could not parse, bro");
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    // The main goal here is to convert a string into a Circle using the parse method.
    let radius = "   3    ";
    let circle: Circle = radius.parse().expect("Nah, hell naw!!");
    println!("{:?}", circle);
}
