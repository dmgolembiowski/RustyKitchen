// Lines 2-5 happen automatically in the prelude!
enum _Option<T> {
    Some(T),        // Option::Some
    None,           // Option::None <-- Rust's null value
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    /* Note:
     * -----
     *
     * If using `None`, need to tell Rust compiler
     * which generic type of None it is covering for,
     * b/c otherwise it cannot properly infer the type.
     */
}
