fn main() {
    // Have caution,
    // it's generally a bad idea to try to slice strings
    // and recover characters by index slice,
    // because doing something like:
    let hello = "3abcdefghijkl";
    let s = &hello[0..3];
    println!("&hello[0..4] = {}", s);
}
