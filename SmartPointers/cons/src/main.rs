enum List {
    Cons(i32, Box<List>), // Could have done: Cons(T, List)
    Nil,
}
use crate::List::{Cons, Nil};

#[allow(dead_code)]
fn main() {
    let _list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
