fn main() {
    // Integer Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer Subtraction
    println!("1 - 2 = {}", 1i32 + 2);

    // Scientific Notation
    println!("ie4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short circuiting boolean logic
    println!("True AND false is {}", true && false);
    println!("True OR false is {}", true || false);
    println!("Not true is {}", !true);

    // Bitwise
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101u32);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
