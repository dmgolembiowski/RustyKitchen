pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub repr: Vec<f64>,
}

impl Draw for Button {
    fn draw(&self, x1: f64, x2: f64, y1: f64, y2: f64) {
        self.repr = vec![x1, y1, x2, y2];
    }


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
