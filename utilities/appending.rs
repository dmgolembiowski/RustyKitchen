fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    // Results in "foobar"
   s.push('y');
   // Res: foobary
   
   // To iterate over a string, do:
   let alphabet = "abcdefghijklmnopqrstuvwxyz";
   for letter in alphabet.chars() {
        println!("{}", letter);
   }
    // Alternatively, can iterate over each of the bytes
    for nibble in alphabet.bytes() {
        println!("{}", nibble);
    }
}
