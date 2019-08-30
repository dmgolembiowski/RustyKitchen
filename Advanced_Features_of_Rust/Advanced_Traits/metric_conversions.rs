use std::ops::Add;

#[derive(Debug)]
struct Millimeters(f32);
#[derive(Debug)]
struct Meters(f32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000 as f32))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;

    fn add(self, other: Millimeters) -> Meters {
        Meters(self.0 + (other.0 / 1000 as f32))
    }
}

fn main() {
    let first = Millimeters(12.0);
    let second = Millimeters(1_068.0);
    let third = Meters(90.0);

    println!("{:?}", second + third as Meters);
}
