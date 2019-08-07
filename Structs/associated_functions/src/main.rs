// Continuing this theme of Rectangles as structs, we have
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    // but why is this allowed, Rust devs?
    fn also_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // sigh
    let b_there_or_b = Rectangle::square(4);
    let c_u_ther = Rectangle::also_square(9);

}
