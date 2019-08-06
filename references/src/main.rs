// This is where all of the Rusty magic happens!
// (not by the power of friendship, 
//  nor the fervent wish to will to happen,
//  nay, but how it ACTUALLY happens: by the '&' character.)
fn other_main() -> String {
    let mut s = String::from("hello");
    change(&mut s);
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Now for other main:
    let s2: String = other_main();
    println!("s2 is: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

