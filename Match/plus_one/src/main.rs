/*
fn plus_two(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    let z = Some(x+y);
    match z {
        None => None,
        Some(i + j) = > Some(i + j + 2),
    }
}
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // covers 8 to 255
    }

}
