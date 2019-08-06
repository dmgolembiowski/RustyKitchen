#[allow(dead_code)]
struct User {
    nickname: String,
    online: bool,
}

/* Note:
 * -----
 * Cannot do:
 *
 *      `struct mut User`, 
 *      or `mut struct User`
 * */

fn main() {
    let mut jeff = User {
        nickname: String::from("jeff"),
        online: false,
    };

    jeff.nickname = String::from("jeffy");
}
