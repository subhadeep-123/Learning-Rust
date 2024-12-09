// Use type to rename types; use UpperCamelCase, except for primitives.

// NanoSecond, Inch and U64 are new names for u64.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // NanoSecond = Inch = U64 = u64;
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // Types aliases do not provide any extra type safety
    // because aliases are not new types
    println!(
        "{} nanosecond + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
