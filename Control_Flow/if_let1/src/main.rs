fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        // A match that only cares about `some_u8_value`:=`Some(3)`
        Some(3) => println!("three"),
        _ => (),
    }

    // Can turn lines 2..7 into:
    if let Some(3) = some_u8_value {
        println!("three");
    }

}
