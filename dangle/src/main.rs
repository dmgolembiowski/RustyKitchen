/* This one raises an error, because you cannot return a borrowed value (&)
fn dangling() -> &String {
    let s = String::from("hello");
    &s
*/

fn dangling() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    let ref_to_nothing = dangling();
}
