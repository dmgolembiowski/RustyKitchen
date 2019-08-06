

fn main_two() -> <(&str, &str)> {
    let mut s = "hell no";
    let mut r1 = &s[0..3];
    let mut r2 = &s[5..6];

    // Testing if my syntax is correct for chaining a ref
    (r1, r2)
}

fn main() {
    println!("true AND false is {}", true && false);
    
    println!("true OR  false is {}", true || false);

    println!("NOT true is {}", !true);

    //BITWISE STUFF
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    
    println!("0011 XOR 0101 is {:04b}",  0b0011u32 ^ 0b0101);

    // shift left operator moves the bits to the left,
    // discards the far left bit, and assigns the 
    // rightmost bit a value of 0.
    // Each move to the left effectively multiplies the 1 by 2 five times,
    // or just 1 * 2**5
    println!("1 << 5 is {}", 1u32 << 5);
    
    // shift right operator moves the bits to the right, discards
    // the far right bit, and assigns the leftmost bit a value of 0.
    // Each move to the right effectively divides 0x80 in half,
    // in this case twice. (0x80 -> 0x40 -> 0x20)
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    
    println!("Use underscores to improve readability, like 1_000_000u32 = {}", 1_000_000u32);

    let tuple1 = main_two();
}
