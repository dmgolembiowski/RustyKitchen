// One use of `enum` is to create distinctions
// between two things, like say "A" and "B", where
// both are fundamentally letters, but also have
// distinct properties and usages.

enum IpAddressProtocol {
    v4,
    v6,
}

struct IpAddress {
    protocol: IpAddressProtocol,
    address:  String,

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
    let localhost = IpAddress {
        protocol: IpAddressProtocol::v4,
        address:  String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        protocol: IpAddressProtocol::v6,
        address: String::from("::1"),
    };
}
