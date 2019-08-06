fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn main() {
    let s1 = gives_ownership(); // s1 <-- return
    let s2 = String::from("hello");
    let fakeS2 = s2.clone();
    let s3 = takes_and_gives_back(s2);
    // println!("({},{},{})", s1, s2, s3);
    // ^ Attempting this line results in:
    // borrow of moved value: `s2`  value borrowed here after move [E0382]
    println!("({},{},{})", s1, fakeS2, s3);


}
