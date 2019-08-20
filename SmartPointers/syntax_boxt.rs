/// Storing an i32 value on the heap using a box
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
