/* 
 * The are three types of `structs` in Rust, which
 * can be created using the ```struct``` keyword:
 *  - Tuple structs, which are NamedTuples
 *  - The classic C structs
 *  - Unit structs, which are fieldless, and are useful for generics
 */

// NamedTuple-y One
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct: typically used for type parameters
// https://doc.rust-lang.org/rust-by-example/generics.html
struct Nil;


// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another, different Struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
/* `main()`
 *
 * A lot happens in here, including a variety of ways to assign data,
 * build "class instances", which are now called ```structs```*/
fn main() {
    // Singleton structure creation with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print the debug struct for `peter`
    println!("{:?}", peter);


    // Instantiate this "point" from `Point`
    // Recognized as: `let point = [new] Point { ... };`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // self.attribute ...or this.property
    println!("point coordinates: ({}, {})", point.x, point.y);


    // Use the struct-update syntax to recycle fields
    // from a previously defined struct of the same "Class"
    // . Written as `newly_declared = ... { param1: $val, ..prev_declared };`
    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);


    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression, too
        // (so... no return statement is needed then?)
        p1: Point { x: my_y, 
                    y: my_x },
        p2: point,
    };

    // Declare/Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Acess the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}


