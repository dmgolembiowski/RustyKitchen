struct Point<T> {
    x: T,
    y: T,
}

struct MultiTypePoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1, y: 2 };
    // This next one won't work:
    // let wont_work = Point {x:5, y:1.0};
    // b/c contradicts the single type T
    let does_work = MultiTypePoint{x:5,y:4.0};

}
