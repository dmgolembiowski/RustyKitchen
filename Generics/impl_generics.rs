struct Point<T> {x:T, y:T}
impl<T> Point<T> { 
    fn x(&self) -> &T {
        &self.x
    }
    // Nevermind, not as cool as I thought;
    // they can't share a name... :(
    /*
    fn distance_from_origin(&self) -> &T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    */
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point {x:5, y:10};
    println!("p.x = {}", p.x());
    let p1 = Point {x: 5.0, y: 10.0};
    let dist: f32 = p1.distance_from_origin();
    println!("dist = {:?}", dist);
}
