fn main() {
    let mut v = vec![2, 3, 5, 7, 11];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    println!("Hello, world!");
}
