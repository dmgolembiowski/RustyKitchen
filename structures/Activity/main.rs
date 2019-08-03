/* 
 * The are three types of `structs` in Rust, which
 * can be created using the ```struct``` keyword:
 *  - Tuple structs, which are NamedTuples
 *  - The classic C structs
 *  - Unit structs, which are fieldless, and are useful for generics
 */
// A struct with two fields

struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    fn as_tuple(&self) -> (x: f64, y: f64) {
        let repr = (self.x, self.y);
        return repr;
    }
}

#[allow(dead_code)]
struct Rectangle { p1: Point, p2: Point, p3: Point, p4: Point }

impl Rectangle {
    fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Rectangle {
        Rectangle { p1: p1, p2: p2,
                    p3: p3, p4: p4 }
    }

    // Using `&self` instead of `self: &Self`
    fn points(&self) {
        let mut list = vec![ self.p1.as_tuple(), self.p2.as_tuple(),
                             self.p3.as_tuple(), self.p4.as_tuple()];
    } 
    
    // Change back to `rect_area(&self) -> f64` once complete
    fn assemble(&self) {
        let pairs = self.points();
        for pair in &pairs {
            println!("{}", pair);
        }
    }
}  
fn main() {
    // Points on a rectangle
    let point_one: Point = Point { x: 1.0, y: 1.0 };
    let point_two: Point = Point { x: 1.0, y: 3.0 };
    let point_three: Point = Point { x: 2.0, y: 1.0 };
    let point_four: Point = Point { x: 2.0, y: 3.0 };

    // Define an arbitrary yellow rectangle
    let spongebob = Rectangle(point_one, point_two, point_three, point_four);
}


