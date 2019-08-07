struct Ipv4Addr {
    // resembles this
}

struct Ipv6Addr {
    // stuff in here
}

enum IpAddr {
    V4(Ipv4Addr),
    v6(Ipv6Addr),
}

fn main() {
    println!("Hello, world!");
}
