// Let's compare usage of associated type with generic
/*
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter { value: u32 }

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {}
}

/* Seems so similar, so why not define a generic, as: */
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}
*/
// With associated types, we don't need to annotate types
// b/c we can't implement a trait on a type multiple times.
//
// There will only be one `impl Iterator for Counter`
//

/// Default Generic Type Parameters and Operator Overloading
/* Follows Syntax: <PlaceholderType=ConcreteType>
 */
// This is useful for operator overloading, like changing the behavior of +
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
fn main() {
    let first:Point = Point {x:1, y:0};
    let second:Point = Point {x:2, y:3};
    let third:Point = Point {x:3, y:3};
    assert_eq!(first+second, third);
}
