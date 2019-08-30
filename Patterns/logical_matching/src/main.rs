fn main() {
    let x = 4; 
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("y is true!"),
        _ => println!("wrong"),
    }
}

fn _main2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50 dollas"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
