// This is where all of the Rusty magic happens!
// (not by the power of friendship, 
//  nor the fervent wish to will to happen,
//  nay, but how it ACTUALLY happens: by the '&' character.)
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

