fn main() {
    // s comes into scope
    let s = String::from("hello");

    takes_ownership(s);
    /* s's value moves into function and
     * so it's no longer valid here*/

    // assert!(s.ends_with("o"));
    // ^ borrow of moved value: `s`  value borrowed here after move [E0382]
    
    let x = 5;
    
    makes_copy(x);
    /* x would move into the function, but i32 is Copy
     * so it's okay to still use x afterward*/
}
/* Here, x goes out of scope, then s. But since s's value
 * was moved nothing special happens
 * */
fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}

// `some_string.drop()` gets called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    // some int enters scope
    println!("{}", some_integer);
}


