#[derive(Debug)]
struct Rectangle {
    width: f32, 
    height: f32,
}

impl Rectangle {
    fn enlarge(&mut self) {
        self.width = 2.5 * self.width;
        self.height = 2.5 * self.height;
    }
}

fn main() {
    let mut rect = Rectangle { width: 30.0 , height: 50.0 };
    rect.enlarge();
    println!("{}", rect.width * rect.height);
}
