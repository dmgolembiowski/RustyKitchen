// One use of `enum` is to create distinctions
// between two things, like say "A" and "B", where
// both are fundamentally letters, but also have
// distinct properties and usages.

enum IpAddress {
    v4(u8, u8, u8, u8),
    v6(String),
}


/* A more concise way:
 *
 * enum IpAddr {
 *     v4(String),
 *     v6(String),
 * }
 *
 * let home = IpAddr::v4(String::from("127.0.0.1"));
 * let loopback = IpAddr::v6(String::from("::1");
 */
fn main() {
    let localhost = IpAddress::v4(127, 0, 0, 1);

    let loopback = IpAddress::v6(String::from("::1"));
}
