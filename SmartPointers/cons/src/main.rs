enum List {
    Cons(i32, Box<List>), // Could have done: Cons(T, List)
    Nil,
}

enum Newlist {
    Newcons(i32, Rc<Newlist>),
    Newnil,
}

use crate::Newlist::{Newcons, Newnil};
use crate::List::{Cons, Nil};
use std::rc::Rc;

#[allow(dead_code)]
fn main() {
    let _list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

#[test]
fn new_main() {
    let a = Rc::new(Newcons(5, Rc::new(Newcons(10, Rc::new(Newnil)))));
    let _b = Newcons(3, Rc::clone(&a));
    let _c = Newcons(4, Rc::clone(&a));
}

