use std::fmt;

#[allow(dead_code)]
fn main() {
    struct Color(i32, i32, i32);
    struct Point {
        x: f32,
        y: f32,
        z: f32,
    };

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }

    struct Vector3 {
        color: Color,
        point: Point,
        name: String,
    }


    let black = Color(0, 0, 0);
    let z_minus_twenty = Point{ x: 0.0, y: 0.0, z: -20.0};

    let down_twenty = Vector3 {
        color: black,
        point: z_minus_twenty,
        name: String::from("z_minus_twenty"),
    };

    println!("This Vector3 is named {}", &down_twenty.name);
    println!("{} is located at {}", &down_twenty.name, &down_twenty.point);
}
