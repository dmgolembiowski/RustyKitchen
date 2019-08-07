struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        // I was pretty close, here's the actual working line:
        self.width <= rect2.width && self.height <= rect2.height
    }
}



fn main() {
    let r1 = Rectangle {
        width: 2,
        height: 2,
    };

    let r2 = Rectangle {
        width: 3,
        height: 1,
    };

    let result = r2.can_hold(&r1);
}
