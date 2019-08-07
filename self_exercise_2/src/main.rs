struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, rect2: &Rectangle) {
        if self.width <= rect2.width && self.height <= rect2.height {
            true;
        } else {
            false;
        }
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
    result
}
