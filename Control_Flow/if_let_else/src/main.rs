#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    /* Demonstration of syntax */

    // Using a match expression
    let mut count = 0;
    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Using an if-let-else expression:
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
