struct Point {
    x: i32,
    y: i32,
}

fn foo(_: i32, y: i32 ) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    let ((feet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: -10 });
    foo(3, 4);

    // Extra Conditionals with Match Guard
    let num = Some(4);

    match num {
        Some(x) if x < 5 => (),
        Some(x) => println!("{}",x),
        None => (),
    }
}
