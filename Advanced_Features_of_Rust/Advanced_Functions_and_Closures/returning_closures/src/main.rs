fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn main() {
    let add_one = returns_closure();
    let six: i32 = add_one(5);
    println!("Did it work? {}", six);
}
