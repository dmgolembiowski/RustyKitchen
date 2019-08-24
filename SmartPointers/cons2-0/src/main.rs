use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

/// Struct std::cell::RefCell
/// -------------------------
/// A mutable memory location with dynamically checked borrow rules.
///
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


fn main() {
    /* `value` is a single-threaded reference-counting pointer,
    providing shared ownership of a value type T, allocated in the heap.
    */
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(
        Cons(
            Rc::clone(&value),
            Rc::new(Nil)
        )
    );

    let b = Cons(
        Rc::new(RefCell::new(6)),
        Rc::clone(&a),
    );

    let c = Cons(
        Rc::new(RefCell::new(10)),
        Rc::clone(&a),
    );



    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
